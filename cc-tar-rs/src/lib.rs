use log::debug;
use std::{io::Read, str::from_utf8};

use crate::{constants::DEFAULT_BLOCK_SIZE_BYTES, io::read_source_512b};

pub mod constants;
mod io;

#[derive(Default)]
pub struct Archive {
    contents: Vec<ArchivedFile>,
}

impl Archive {
    pub fn list(&self) -> &Vec<ArchivedFile> {
        &self.contents
    }

    pub fn add_file(&mut self, file: ArchivedFile) {
        self.contents.push(file);
    }
}

#[derive(PartialEq, Debug)]
pub enum ArchivedFileType {
    NormalFile,
    HardLink,
    SymbolicLink,
    CharacterSpecial,
    BlockSpecial,
    Directory,
    Fifo,
    Unknown,
}

#[derive(Debug)]
pub struct ArchivedFile {
    pub file_name: String,
    pub file_size: usize,
    pub file_name_prefix: String,
    pub file_type: ArchivedFileType,
}

// states for the tar state machine
#[derive(PartialEq, Debug)]
pub enum TarSmState {
    FileHeaderRead,
    FileContentsRead,
    TerminalBlockRead,
    FileHeaderWrite,
    FileContentsWrite,
    TerminalBlockWrite,
    EndOfArchive,
}

// state machine representing archive processing
#[derive(Debug)]
pub struct TarStateMachine {
    pub state: TarSmState,
    pub num_blocks_loaded: usize,
    pub current_block_num: usize,
    pub current_file_blocks_remaining: usize,
}

pub fn read_archive<T: Read>(source: &mut T) -> Archive {
    let mut sm = TarStateMachine {
        state: TarSmState::FileHeaderRead,
        num_blocks_loaded: 0,
        current_block_num: 0,
        current_file_blocks_remaining: 0,
    };

    let mut archive: Archive = Archive {
        contents: Vec::new(),
    };
    // A tar archive is made of:
    // - 1 512-byte block for the header
    // - file_size in bytes / 512-bytes for the number of complete blocks for the file
    // - 1 padded block for the last partially filled 512 byte block
    //
    // For a 15 byte file:
    // - starting at the 0th block (header is 512 bytes)
    // - the next block is a partially filled 512-byte block
    // - next block index should be 0 + 1 + 1 = 2
    loop {
        let block = read_source_512b(source);
        match block {
            Ok(byte_vec) => {
                if byte_vec.is_empty() {
                    panic!("Unexpected EOF");
                }
                sm.num_blocks_loaded += 1;
                sm.current_block_num = sm.num_blocks_loaded;
                process_archive_block(&byte_vec, &mut sm, &mut archive);
            }
            Err(err) => {
                panic!("failed to read block from source {:?}", err);
            }
        }
        if sm.state == TarSmState::EndOfArchive {
            break;
        }
    }

    archive
}

fn process_archive_block(data: &[u8], tar_sm: &mut TarStateMachine, archive: &mut Archive) {
    debug!("state: {:?}", tar_sm);
    match tar_sm.state {
        TarSmState::FileHeaderRead => {
            // The state machine is currently looking to process a file header block.
            let archived_file = process_archive_file_header(data);
            match archived_file {
                Some(file) => {
                    debug!("processed archived file: {:?}", file);
                    if file.file_type == ArchivedFileType::NormalFile {
                        // compute the number of file blocks to read
                        let num_content_blocks = calculate_blocks_for_file_contents(&file);
                        tar_sm.current_file_blocks_remaining = num_content_blocks;
                        tar_sm.state = TarSmState::FileContentsRead;
                    }

                    archive.contents.push(file);
                }
                None => {
                    debug!("did not find a file in the header block");
                    // The block is empty/corrupt. Treat it as a terminal for now.
                    // TODO: improve corrupt block detection. Empty != corrupt.
                    tar_sm.state = TarSmState::TerminalBlockRead;
                }
            }
        }
        TarSmState::FileContentsRead => {
            // The state machine is currently looking to process a file contents block.
            // This is a no-op for now as we don't want to store file contents anywhere.
            // In future, this will write to a file during archive extraction if required.
            tar_sm.current_file_blocks_remaining -= 1;
            if tar_sm.current_file_blocks_remaining == 0 {
                // All file blocks have been processed, we can go back to header mode.
                tar_sm.state = TarSmState::FileHeaderRead;
            }
        }
        TarSmState::TerminalBlockRead => {
            // The state machine has already detected a terminal block.
            // tar archives always have atleast 2 512-byte blocks at the end which are empty.
            // For a file with 15 bytes, there will be a minimum of 4 512-byte blocks.
            // Due to blocking factor, there will always be a minimum of 20 blocks in the archive.
            // ref: https://www.gnu.org/software/tar/manual/html_node/Blocking-Factor.html
            // If there are 2 subsequent empty blocks seen, it is considered as end-of-archive.
            let archived_file = process_archive_file_header(data);
            match archived_file {
                Some(_) => {
                    panic!("unexpected file header block");
                }
                None => {
                    // End of Archive
                    tar_sm.state = TarSmState::EndOfArchive;
                }
            }
        }
        _ => {}
    }
}

fn calculate_blocks_for_file_contents(archived_file: &ArchivedFile) -> usize {
    let num_full_blocks_for_file = archived_file.file_size / DEFAULT_BLOCK_SIZE_BYTES;
    let num_partial_blocks_for_file = if !archived_file
        .file_size
        .is_multiple_of(DEFAULT_BLOCK_SIZE_BYTES)
    {
        1
    } else {
        0
    };
    let num_content_blocks_for_file = num_full_blocks_for_file + num_partial_blocks_for_file;
    debug!(
        "num content blocks for file: {}",
        num_content_blocks_for_file
    );

    num_content_blocks_for_file
}

fn process_archive_file_header(block_data: &[u8]) -> Option<ArchivedFile> {
    let block_start = 0;

    // file name - offset 0 size 100 (0 - 99 inclusive)
    let file_name = from_utf8(&block_data[block_start..(block_start + 100)])
        .unwrap()
        .trim_matches(char::from(0))
        .to_string();
    if file_name.is_empty() {
        debug!("file name is empty");
        // if the file name is empty, then it is an empty block
        return None;
    }
    debug!("current file name: {}", file_name);

    // file size - offset 124 size 12 (124 - 135 inclusive)
    let file_size_start = block_start + 124;
    let file_size_str = from_utf8(&block_data[file_size_start..(file_size_start + 12)])
        .unwrap()
        .trim_matches(char::from(0))
        .to_string();
    debug!("current file size: {}", file_size_str);
    let file_size = usize::from_str_radix(&file_size_str, 8).unwrap();

    // file type - offset 156 size 1
    let file_type_start = block_start + 156;
    let file_type_str = from_utf8(&block_data[file_type_start..(file_type_start + 1)])
        .unwrap()
        .to_string();
    debug!("current file type string: {}", file_type_str);
    let file_type = match file_type_str.as_str() {
        "0" => ArchivedFileType::NormalFile,
        "1" => ArchivedFileType::HardLink,
        "2" => ArchivedFileType::SymbolicLink,
        "3" => ArchivedFileType::CharacterSpecial,
        "4" => ArchivedFileType::BlockSpecial,
        "5" => ArchivedFileType::Directory,
        "6" => ArchivedFileType::Fifo,
        _ => ArchivedFileType::Unknown,
    };

    // file name prefix - offset 345 size 155 (345 to 499 inclusive)
    let prefix_start_size = block_start + 345;
    let file_prefix = from_utf8(&block_data[prefix_start_size..(prefix_start_size + 155)])
        .unwrap()
        .trim_matches(char::from(0))
        .to_string();
    debug!("current file prefix: {}", file_prefix);

    let file = ArchivedFile {
        file_name,
        file_size,
        file_name_prefix: file_prefix,
        file_type,
    };

    Some(file)
}
