// path_handler.rs
use std::path::PathBuf;
use std::env;
use dirs;
// get current directory: https://doc.rust-lang.org/std/env/fn.current_exe.html
use std::env::current_exe;


const PROGRAM_PATH_FILE: &str = "ProgramPaths.json";
const PROFILES_FILE_NAME: &str = "MinecraftPlayProfiles.json";
const RUN_FILE_NAME: &str = "run_settings.cfg";
const RUN_BAT_FILE_NAME: &str = "RunMinecraft.bat";

const PROGRAM_FOLDER_NAME: &str = "MinecraftProfileManager";

// function to return path to program config folder
// such as ~/.config/MinecraftProfileManager/
// or      %APPDATA%/MinecraftProfileManager/
pub fn get_config_folder_path() -> PathBuf {
    // get path to parent folder based on platform
    let config_dir = if cfg!(target_os = "windows") {
        // Windows
        let appdata = env::var("APPDATA").expect("APPDATA folder not found!");
        PathBuf::from(appdata).join(PROGRAM_FOLDER_NAME)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        // Mac/Linux
        let home = env::var("HOME").expect("HOME folder not found!");
        PathBuf::from(home).join(".config").join(PROGRAM_FOLDER_NAME)
    } else {
        unimplemented!("Unsupported Operating System! (Unable to make path to minecraft folder)")
    };

    config_dir
}

// function to return path 
pub fn get_profiles_json_path() -> PathBuf {
    // get config folder path
    let config_folder = get_config_folder_path();

    // add json name
    config_folder.join(PROFILES_FILE_NAME)
}

pub fn get_run_settings_path() -> PathBuf {
    // get config folder path
    let config_folder = get_config_folder_path();

    // add run file name
    config_folder.join(RUN_FILE_NAME)
}

pub fn get_run_bat_path() -> PathBuf {
    // get config folder path
    let config_folder = get_config_folder_path();

    // add bat name
    config_folder.join(RUN_BAT_FILE_NAME)
}

// function to get Minecraft path
pub fn get_minecraft_folder() -> PathBuf {
    if cfg!(target_os = "windows") {
        // Windows
        let appdata = env::var("APPDATA").expect("APPDATA folder not found!");
        PathBuf::from(appdata).join(".minecraft")
    } else if cfg!(target_os = "macos") {
        // Mac
        let home = env::var("HOME").expect("HOME folder not found!");
        // Library/Application Support/minecraft
        PathBuf::from(home).join("Library").join("Application Support").join("minecraft")
    } else if cfg!(target_os = "linux") {
        // Linux
        let home = env::var("HOME").expect("HOME folder not found!");
        let user = env::var("USER").expect("USER not found!");
        PathBuf::from(home).join(user).join(".minecraft")
    } else {
        unimplemented!("Unsupported Operating System! (Unable to make path to minecraft folder)")
    }
}

// function for minecraft program paths on different os's
pub fn get_minecraft_program_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        // Windows
        PathBuf::from("C:\\Program Files (x86)\\Minecraft Launcher\\MinecraftLauncher.exe")
    } else if cfg!(target_os = "macos") {
        // Mac
        PathBuf::from("/Applications/Minecraft.app")
    } else if cfg!(target_os = "linux") {
        // Linux
        unimplemented!("Unsupported Operating System! (Linux not supported yet)")
    } else {
        unimplemented!("Unsupported Operating System! (Unable to make path to minecraft executable)")
    }
}