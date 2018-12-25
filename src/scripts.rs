use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?; // todo
    let mut file_content = String::new();
    f.read_to_string(&mut file_content)?;
    Ok(file_content)
}
