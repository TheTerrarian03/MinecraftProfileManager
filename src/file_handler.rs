use std::fs;
use std::io::Write;
// file_handler.rs
use std::{fs::File, io::Read};
use std::path::Path;
use crate::{path_handler, data_handler};


pub fn validate_files() -> Result<(), String> {
    // check if minecraft exists
    let mc_path = path_handler::get_minecraft_folder();
    if !mc_path.exists() {
        return Err("No Minecraft Path".to_string());
    }

    // check if profiles json exists at the required path
    let profiles_path = path_handler::get_profiles_json_path();
    if !profiles_path.exists() {
        // doesn't exist, try to write default profile to json
        write_default_profiles_json();
    }

    // all good
    Ok(())
}

// function to create the default profiles json file
// WORKS!
pub fn write_default_profiles_json() {
    // get expected path to json file
    let profiles_path = path_handler::get_profiles_json_path();

    println!("{:?}", profiles_path);

    // get parents of that json file path (config folder)
    let config_folder = path_handler::get_config_folder_path();

    // make sure necessary folders exist/have been made
    fs::create_dir_all(config_folder).expect("Failed to make neccessary folders for profile json");

    // file obj
    let mut file = File::create(profiles_path).expect("Unable to open path to profiles json");

    // write defaults to json file
    file.write_all(data_handler::DEFAULT_PROFILES_DATA.as_bytes()).expect("Failed to write to profiles json");
}

/// Generic file methods
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
