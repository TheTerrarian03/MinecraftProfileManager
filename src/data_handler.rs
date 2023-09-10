// to read/write from/to json files
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::path_handler;


/// Structs for data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunOptions {
    pub run_offline: bool,
    pub change_name: bool,
    pub new_name: String,
    pub auto_click_play: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub run_options: RunOptions,
    pub options: HashMap<String, String>,
    pub optionsshaders: HashMap<String, String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfilesData {
    pub default_profile: String,
    pub profiles: HashMap<String, Profile>
}

/// Constants for default configurations of data
pub const DEFAULT_PROFILES_DATA: &str = r#"
{
    "default_profile": "Profile A",
    "profiles": {
        "Profile A": {
            "run_options": {
                "run_offline": false,
                "change_name": false,
                "new_name": "N/A",
                "auto_click_play": true
            },
            "options": {
                "narrator": "0"
            },
            "optionsshaders": {
                "oldHandLight": "default"
            }
        }
    }
}
"#;

const DEFAULT_PROFILE: &str = r#"
{
    "run_options": {
        "run_offline": false,
        "change_name": false,
        "new_name": "none",
        "auto_click_play": true
    },
    "options": {
        "narrator": "0"
    },
    "optionsshaders": {
        "oldHandLight": "default"
    }
}
"#;

pub const BAT_CONTENT: &str = r#"
@echo off

:: Check for admin access and request if not already elevated
net session >nul 2>&1
if %errorLevel% == 0 (
    echo Admin access granted.
) else (
    echo Requesting admin access...
    powershell -Command "Start-Process -Verb RunAs -FilePath '%0' -ArgumentList 'am_admin'"
    exit /b 0
)

:: Turn Wi-Fi on or off based on the second argument
if "%~2"=="true" (
    netsh interface set interface "Wi-Fi" Disable
    netsh interface set interface "Ethernet" Disable
    echo Wi-Fi is now OFF.
) else (
    echo Invalid argument for Wi-Fi state. Use "true" or "false" as the second argument.
    goto :eof
)

:: Check if an executable path is provided as the first argument
if "%~1"=="" (
    echo Please provide the path to the executable as the first argument.
    goto :eof
)

:: Run the executable
start "" "%~1"

:: Wait for the executable to finish (optional)
:: You can add a timeout or any other logic here if needed
:: Example: ping 127.0.0.1 -n 5 > nul

pause

:: Turn Wi-Fi on or off based on the second argument
if "%~2"=="true" (
    netsh interface set interface "Wi-Fi" Enable
    netsh interface set interface "Ethernet" Enable
    echo Wi-Fi is now ON.
) else (
    echo Invalid argument for Wi-Fi state. Use "true" or "false" as the second argument.
    goto :eof
)
"#;

const DEFUALT_PROFILE_NAME: &str = "New Profile";

pub const DEFUALT_RUN_SETTINGS: &str = "run_offline=false\nchange_name=false\nnew_name=none\nauto_click_play=false\n";

/// functions for reading/writing data
pub fn get_default_profiles_data() -> ProfilesData {
    serde_json::from_str(DEFAULT_PROFILES_DATA).expect("Unable to load default")
    // default_profiles_data
}

pub fn get_default_new_profile() -> Profile {
    serde_json::from_str(DEFAULT_PROFILE).expect("Unable to load default")
}

pub fn write_profiles_data(profiles: &ProfilesData) {
    let json_string = serde_json::to_string_pretty(&profiles).expect("unable to convert profiles to json format");
    let mut file = File::create(path_handler::get_profiles_json_path()).expect("Failed to open profiles json");
    file.write_all(json_string.as_bytes()).expect("Unable to write new data to profiles json");
}

pub fn get_profiles_data() -> ProfilesData {
    let mut file = File::open(path_handler::get_profiles_json_path()).expect("Failed to open profiles json file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read profiles json content");

    let profiles: ProfilesData = serde_json::from_str(&content).expect("Failed to convert json to ProfilesData");
    profiles
}

pub fn does_profile_name_exist(profiles: &ProfilesData, profile_name: &str) -> bool {
    if let Some(_) = profiles.profiles.get(profile_name) {
        true
    } else {
        false
    }
}

/// functions for adjusting existing data
pub fn get_profile_names(profiles: &ProfilesData) -> Vec<String> {
    let profile_names: Vec<String> = profiles.profiles.keys().cloned().collect();
    profile_names
}

pub fn get_data_for_profile(profiles: &ProfilesData, profile_name: &str) -> Result<Profile, String> {
    if let Some(profile_data) = profiles.profiles.get(profile_name) {
        Ok(profile_data.clone())
    } else {
        Err("Nope!".to_string())
    }
}

pub fn get_default_profile_name(profiles: &ProfilesData) -> Result<String, String> {
    if !profiles.default_profile.is_empty() {
        Ok(profiles.default_profile.clone().to_string())
    } else {
        Err("No default!".to_string())
    }
}

pub fn set_new_default_profile_name(profiles: &mut ProfilesData, new_name: &str) {
    profiles.default_profile = new_name.to_string()
}