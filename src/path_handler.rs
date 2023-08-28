// path_handler.rs
use std::path::PathBuf;
use dirs;
// get current directory: https://doc.rust-lang.org/std/env/fn.current_exe.html
use std::env::current_exe;


const INFO_FILE: &str = "INFO_FILE.txt";

// function to return path of the desktop directory
pub fn get_desktop_dir() -> Result<PathBuf, String> {
    if let Some(desktop_dir) = dirs::desktop_dir() {
        Ok(desktop_dir)
    } else {
        Err("Failed to get desktop dir".to_string())
    }
}

// function to return path of the info file (at desktop)
pub fn get_info_file_path() -> Result<PathBuf, String> {
    match get_desktop_dir() {
        Ok(desktop_path) => {
            Ok(desktop_path.join(INFO_FILE))
        },
        Err(error) => Err(error)
    }
}