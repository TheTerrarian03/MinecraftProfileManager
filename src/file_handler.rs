// file_handler.rs
use std::{fs::File, io::Read};


pub fn read_file(file_path: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}