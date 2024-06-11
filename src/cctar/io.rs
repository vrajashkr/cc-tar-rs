use std::{fs::File, io::{stdin, Error, Read}};

pub fn read_stdin() -> Result<Vec<u8>, Error>{
    read_source(stdin())
}

pub fn read_file(path: &str) -> Result<Vec<u8>, Error>{
    let f = File::open(path)?;
    read_source(f)
}

fn read_source<T: Read>(mut file: T) -> Result<Vec<u8>, Error>{
    let mut contents: Vec<u8> = Vec::new();
    let mut buffer = [0; 1024];
    loop {
        let read_result = file.read(&mut buffer);
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
