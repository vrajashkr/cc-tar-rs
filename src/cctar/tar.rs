use std::{io::{self, Error, Read}, str::from_utf8};

use crate::config::config::{self, Config};

use super::{constants::DEFAULT_BLOCK_SIZE_BYTES, types::{Archive, ArchivedFile}};

pub fn run_tar(config: &Config) {
    match config.mode {
        config::Mode::List => {
            list_contents(config)
        },
        config::Mode::Create => {
            println!("Not supported yet!")
        }
    }
}

fn list_contents(_: &Config) {
    let file_contents = read_stdin().unwrap_or_else(|err| {
        panic!("failed to read from stdin {:?}", err);
    });

    let archive = process_archive(&file_contents);
    for archived_file in archive.contents {
        println!("{}{}", archived_file.file_name_prefix, archived_file.file_name);
    }
}

fn process_archive(data: &Vec<u8>) -> Archive {
    let mut archive: Archive = Archive { contents: Vec::new() };
    let mut current_block_num: usize = 0;
    let mut empty_terminal_block_count: usize = 0;

    loop {
        // A tar archive is made of:
        // - 1 512-byte block for the header
        // - file_size in bytes / 512-bytes for the number of complete blocks for the file
        // - 1 padded block for the last partially filled 512 byte block
        //
        // For a 15 byte file:
        // - starting at the 0th block (header is 512 bytes)
        // - the next block is a partially filled 512-byte block
        // - next block index should be 0 + 1 + 1 = 2
        match process_archive_file(data, current_block_num) {
            None => {
                // the block was empty
                empty_terminal_block_count += 1;
                if empty_terminal_block_count == 2 {
                    // end of archive
                    // tar archives always have atleast 2 512-byte blocks at the end which are empty.
                    // For a file with 15 bytes, there will be a minimum of 4 512-byte blocks.
                    // Due to blocking factor, there will always be a minimum of 20 blocks in the archive.
                    // ref: https://www.gnu.org/software/tar/manual/html_node/Blocking-Factor.html
                    // if there are 2 subsequent empty blocks seen, it is considered as end-of-archive.
                    break;
                }
                // Since the last block was empty, move to the next block.
                current_block_num += 1;
            },
            Some(archived_file) => {
                let mut block_count_for_file = 1 + (&archived_file.file_size / DEFAULT_BLOCK_SIZE_BYTES);
                if &archived_file.file_size % DEFAULT_BLOCK_SIZE_BYTES > 0 {
                    block_count_for_file += 1;
                }
                current_block_num += block_count_for_file;
                archive.contents.push(archived_file);
            }
        }
    }

    archive
}

fn process_archive_file(data: &[u8], block_num: usize) -> Option<ArchivedFile> {

    // This is the offset at which to start processing a file's blocks.
    // This should be the offset for the file header.
    let block_start = block_num * DEFAULT_BLOCK_SIZE_BYTES;

    // file name - offset 0 size 100 (0 - 99 inclusive)
    let file_name = from_utf8(&data[block_start .. (block_start + 100)])
                            .unwrap().trim_matches(char::from(0)).to_string();
    if file_name.is_empty() {
        // if the file name is empty, then it is an empty block
        return None
    }

    // file size - offset 124 size 12 (124 - 145 inclusive)
    let file_size_start = block_start + 124;
    let file_size_str = from_utf8(&data[file_size_start .. (file_size_start + 12)])
                                .unwrap().trim_matches(char::from(0)).to_string();
    let file_size = usize::from_str_radix(&file_size_str, 8).unwrap();

    // file name prefix - offset 345 size 155 (345 to 499 inclusive)
    let prefix_start_size = block_start + 345;
    let file_prefix = from_utf8(&data[prefix_start_size .. (prefix_start_size + 155)])
                                .unwrap().trim_matches(char::from(0)).to_string();

    let file = ArchivedFile{
        file_name,
        file_size,
        file_name_prefix: file_prefix,
    };

    Some(file)
}

fn read_stdin() -> Result<Vec<u8>, Error>{
    let mut contents: Vec<u8> = Vec::new();
    let mut buffer = [0; 1024];
    loop {
        let read_result = io::stdin().read(&mut buffer);
        match read_result {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                contents.extend(&buffer[0 .. n]);
            },
            Err(e) => { return Err(e);}
        }
        
    }
    Ok(contents)
}
