use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
// file_handler.rs
use std::{fs::File, io::Read};
use std::path::{Path, self, PathBuf};
use crate::data_handler::Profile;
use crate::{path_handler, data_handler};
use serde_json::Value;


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

    // check if run_settings.cfg file exists
    let run_settings_path = path_handler::get_run_settings_path();
    if !run_settings_path.exists() {
        // doesn't exist, try to write default contents to file
        write_default_run_settings();
    }

    // check if run bat file exists
    let run_bat_path = path_handler::get_run_bat_path();
    if !run_bat_path.exists() {
        // doesn't exist, try to make file
        make_run_bat_file(&run_bat_path);
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

pub fn write_default_run_settings() {
    // get expected path to file
    let run_settings_path = path_handler::get_run_settings_path();

    // get parent folder of that path (config folder)
    let config_folder = path_handler::get_config_folder_path();

    // make sure necessary folders exist/have been made
    fs::create_dir_all(config_folder).expect("Failed to make neccessary folders for run settings cfg");

    // file obj
    let mut file = File::create(run_settings_path).expect("Unable to open path to run settings cfg");

    // write default content to file
    file.write_all(data_handler::DEFUALT_RUN_SETTINGS.as_bytes()).expect("Failed to write to run settings cfg");
    
}

/// Minecraft-specific methods
// Main func, calls other funcs, returns true if there was an error
pub fn write_config_to_minecraft(profile: &Profile) -> bool {
    let mut errors: u32 = 0;
    errors += write_run_settings(&profile.run_options);
    if profile.run_options.change_name {
        errors += write_acc_name(&profile.run_options.new_name);
    }
    errors += write_options_s_settings("options.txt", &profile.options, ":");
    errors += write_options_s_settings("optionsshaders.txt", &profile.optionsshaders, "=");
    
    if errors > 0 {
        return true
    }
    false
}

// write run settings to config.ini in config path
fn write_run_settings(run_options: &data_handler::RunOptions) -> u32 {
    // path to file
    let run_settings_path = path_handler::get_run_settings_path();
    
    // clear cfg file
    clear_file(&run_settings_path).expect("Failed to clear run_settings.cfg");

    // file to write to
    let mut file = File::create(run_settings_path).expect("Failed to create run_settings.cfg");

    // write settings
    file.write_all(format!("run_offline={}\n", run_options.run_offline.to_string()).as_bytes()).expect("Failed to write data to run_settings.cfg");
    file.write_all(format!("change_name={}\n", run_options.change_name.to_string()).as_bytes()).expect("Failed to write data to run_settings.cfg");
    file.write_all(format!("new_name={}\n", run_options.new_name).as_bytes()).expect("Failed to write data to run_settings.cfg");
    file.write_all(format!("auto_click_play={}\n", run_options.auto_click_play.to_string()).as_bytes()).expect("Failed to write data to run_settings.cfg");

    0
}

// write account data for manual name
fn write_acc_name(new_name: &String) -> u32 {
    // file path to load
    let launcher_acc_path = path_handler::get_minecraft_folder().join("launcher_accounts.json");

    // Load the launcher_accounts.json file in read mode
    let mut acc_file = File::open(&launcher_acc_path).expect("Unable to load launcher accounts file");
    // read the data from the file using some ChatGPT magic I have no idea what is happening    
    let mut data_str = String::new();
    acc_file.read_to_string(&mut data_str).expect("Unable to read launcher accounts data to string");
    let mut data: Value = serde_json::from_str(&data_str).expect("Unable to convert launcher accounts data to json");

    println!("{}", data);

    // Collect account IDs
    let account_ids: Vec<String> = data["accounts"].as_object().unwrap().keys().cloned().collect();

    // Update names
    for account_id in &account_ids {
        if let Some(account) = data["accounts"].get_mut(account_id) {
            if let Some(mc_profile) = account["minecraftProfile"].as_object_mut() {
                mc_profile.insert("name".to_string(), Value::String(new_name.to_string()));
            }
        }
    }

    // clear file
    clear_file(&launcher_acc_path).expect("Unable to clear json file");

    // close file and re-open in write mode
    drop(acc_file);
    let mut acc_file = File::create(&launcher_acc_path).expect("Unable to open launcher accounts file");

    acc_file.write_all(serde_json::to_string_pretty(&data).expect("Unable to convert json data to string").as_bytes()).expect("Unable to write data to launcher accounts file");
    
    0
}

// write options.txt or optionsshaders.txt settings
fn write_options_s_settings(file_name: &str, custom_options: &HashMap<String, String>, seperator: &str) -> u32 {
    // path to file
    let file_path = path_handler::get_minecraft_folder().join(file_name);

    // existing content to go over
    let file_content = fs::read_to_string(&file_path).expect("Failed to open the intended options file");

    // new content to write out
    let mut new_options_data = String::new();

    // loop to go over lines and options
    for line in file_content.lines() {
        let mut overwritten = false;
        for (cust_option, cust_value) in custom_options {
            if line.starts_with(cust_option) {
                new_options_data += &format!("{}{}{}\n", cust_option, seperator, cust_value);
                overwritten = true;
            }
        }
        if !overwritten {
            new_options_data += line;
            new_options_data += "\n";
        }
    }

    fs::write(file_path, new_options_data).expect("Failed to write new data to intended options file");

    0
}

// method for making bat file to run
pub fn make_run_bat_file(file_path: &std::path::PathBuf) {
    // get content to write out
    let bat_content = data_handler::BAT_CONTENT;

    // clear file
    clear_file(&file_path).expect("Unable to clear json file");

    // open file
    let mut file = File::create(file_path).expect("Unable to open bat file!");

    // write data
    file.write_all(bat_content.as_bytes()).expect("Unable to write bat content to bat file!");
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
