use std::process::{Command, Stdio};

use crate::path_handler;


pub fn open_minecraft(run_offline: bool, auto_click_play: bool) {
    // if windows, run the bat file
    if cfg!(target_os = "windows") {
        let run_bat_path = path_handler::get_run_bat_path();
        Command::new("cmd")
            .args(&["/C", run_bat_path.to_str().unwrap()])
            .status()
            .expect("Failed running bat file!");
        
        return
    }

    // if needed set offline
    if run_offline { disable_wifi() }

    // run minecraft
    // exe path
    let executable_path = path_handler::get_minecraft_program_path();

    // run console command to open
    if cfg!(target_os = "macos") {
        // Mac
        Command::new("open")
            .arg(executable_path)
            .status()
            .expect("Unable to open Minecraft! (Mac)");
    } else if cfg!(target_os = "linux") {
        // Linux
        unimplemented!("Unsupported Operating System! (Linux not supported yet)")
    } else {
        unimplemented!("Unsupported Operating System! (Unable to make path to minecraft executable)")
    }

    // auto click play
    

    // if needed set online
    if run_offline { enable_wifi() }
}

fn disable_wifi() {
    if cfg!(target_os = "macos") {
        // Mac
        Command::new("networksetup")
            .args(&["-setairportpower", "Wi-Fi", "off"])
            .status()
            .expect("Unable to disable wifi");
    } else if cfg!(target_os = "linux") {
        // Linux
        unimplemented!("Unsupported Operating System! (Linux not supported yet)")
    } else {
        unimplemented!("Unsupported Operating System! (Unable to make path to minecraft executable)")
    }
}

fn enable_wifi() {
    if cfg!(target_os = "macos") {
        // Mac
        Command::new("networksetup")
            .args(&["-setairportpower", "Wi-Fi", "on"])
            .status()
            .expect("Unable to disable wifi");
    } else if cfg!(target_os = "linux") {
        // Linux
        unimplemented!("Unsupported Operating System! (Linux not supported yet)")
    } else {
        unimplemented!("Unsupported Operating System! (Unable to make path to minecraft executable)")
    }
}
