use std::io::{Error, Read};

use log::debug;

use super::constants::DEFAULT_BLOCK_SIZE_BYTES;

pub fn read_source_512b<T: Read>(source: &mut T) -> Result<Vec<u8>, Error> {
    let mut contents: Vec<u8> = Vec::new();
    let mut buffer = [0; DEFAULT_BLOCK_SIZE_BYTES];
    
    let read_result = source.read(&mut buffer);
    match read_result {
        Ok(n) => {
            debug!("read {} bytes from source", n);
            if n != 0 {
                contents.extend(&buffer[0 .. n]);
            }
        },
        Err(e) => { return Err(e); }
    }
    Ok(contents)
}
