use std::process::{Command, Stdio};

use crate::path_handler;


pub fn open_minecraft(run_offline: bool, auto_click_play: bool) {
    // set offline if allowed

    // run minecraft
    // exe path
    // let executable_path = path_handler::get_minecraft_program_path();

    // // run console command to open
    // if cfg!(target_os = "windows") {
    //     // Windows
    //     Command::new("cmd")
    //         .args(&["/C", "start", "", executable_path.to_str().unwrap()])
    //         .status()
    //         .expect("Unable to open Minecraft! (Windows)");
    // } else if cfg!(target_os = "macos") {
    //     // Mac
    //     Command::new("open")
    //         .arg(executable_path)
    //         .status()
    //         .expect("Unable to open Minecraft! (Mac)");
    // } else if cfg!(target_os = "linux") {
    //     // Linux
    //     unimplemented!("Unsupported Operating System! (Linux not supported yet)")
    // } else {
    //     unimplemented!("Unsupported Operating System! (Unable to make path to minecraft executable)")
    // }

    // auto click play
    // not yet

    disable_wifi()
}

fn disable_wifi() {
    if cfg!(target_os = "windows") {
        // Windows
        Command::new("cmd")
            .args(&["/C", ""])
            .status()
            .expect("Unable to open Minecraft! (Windows)");
    } else if cfg!(target_os = "macos") {
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
