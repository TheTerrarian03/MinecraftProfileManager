use std::io::Write;
// file_handler.rs
use std::{fs::File, io::Read};
use std::path::Path;
use crate::path_handler;


const INFO_FILE_DEFUALT: &str = "Some info here!\nEnjoy :D\n-Logan M.";

pub fn validate_files() {
    // let info_file_path = path_handler::get_info_file_path();

    // match info_file_path {
    //     Ok(path) => {
    //         if Path::new(&path).exists() {
    //             println!("INFO FILE successfully checked (exists)");
    //         } else {
    //             println!("INFO FILE does not exist");

    //             match File::create(path) {
    //                 Ok(mut file) => {
    //                     if let Err(err) = file.write_all(INFO_FILE_DEFUALT.as_bytes()) {
    //                         println!("Error making INFO FILE, {}", err)
    //                     } else {
    //                         println!("INFO FILE successfully made")
    //                     }
    //                 },
    //                 Err(err) => println!("There was an error creating INFO FILE, {}", err)
    //             }
    //         }
    //     },
    //     Err(_) => println!("There was an error checking INFO FILE")
    // }
}

pub fn read_file(file_path: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn clear_file(file_path: &std::path::PathBuf) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;

    // ChatGPT on why this works: "Since there's nothing in the byte array, nothing is written to the file. However, the act of 
    //   writing an empty byte array still causes the file to be truncated to zero length, effectively clearing its content."
    file.write_all(b"")?;

    Ok(())
}

pub fn write_lines_to_info_file(file_path: &std::path::PathBuf, lines: Vec<String>) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;

    for line in lines {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}
