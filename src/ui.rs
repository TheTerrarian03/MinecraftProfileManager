// ui module
use std::io;


mod prompts {
    use std::io::{self, Write};

    pub fn input_symbol() {
        print!("$ ");
        io::stdout().flush().expect("Error writing input symbol to console")
    }

    pub fn main_menu() {
        println!("\nMAIN MENU : Please enter an option:");
        println!("1) Read Profiles Data");
        println!("2) Set New Default Profile");
        println!("3) Edit A Profile");
        println!("4) Make New Profile");
        println!("5) Remove A Profile");
        println!("6) Write configuration to Minecraft files");
        println!("7) Run Minecraft");
        println!("8) Quit program");
        input_symbol();
    }

    pub fn read_menu() {
        println!("\nREAD MENU : Here is your text:");
    }

    pub fn new_default_menu() {
        println!("\nDEFAULTS MENU : Please enter the name of the new profile to be default:");
        input_symbol();
    }

    /// EDIT MENU prompts
    pub fn edit_ask_profile() {
        println!("\nEDIT MENU : Please enter the name of the profile you would like to edit:");
        input_symbol();
    }

    pub fn edit_ask_change_run() {
        println!("\nWould you like to edit the run options? (y/n):");
        input_symbol();
    }

    pub fn edit_ask_change_options() {
        println!("\nWould you like to edit the options.txt settings? (y/n):");
        input_symbol();
    }

    pub fn edit_ask_change_optionsshaders() {
        println!("\nWould you like to edit the optionsshaders.txt settings? (y/n):");
        input_symbol();
    }

    pub fn new_profile_menu() {
        println!("\nNEW MENU : Please enter the name of your new profile (the same name as an existing will over-write the old one):");
        input_symbol();
    }

    pub fn remove_profile_menu() {
        println!("\nREMOVE MENU : Please enter the name of the profile you'd like to remove (a non-existent profile will do nothing):");
        input_symbol()
    }
}

mod menus {
    use crate::data_handler;
    use crate::ext_app_handler;
    use crate::path_handler;
    use crate::file_handler;
    use super::prompts;
    use std::collections::HashMap;
    use std::io;
    use std::io::stdin;

    // MAIN MENU
    pub fn process_main_menu_choose(option: u32) -> bool {
        match option {
            1 => { read_menu();                  false },
            2 => { new_default_menu();           false },
            3 => { edit_profile_menu();          false },
            4 => { new_profile_menu();           false },
            5 => { remove_profile_menu();        false },
            6 => { write_menu();                 false },
            7 => { run_menu();                   false },
            8 => { println!("Quitting program"); true  },
            _ => { println!("Invalid input");    false }
        }
    }

    // READ MENU
    pub fn read_menu() {
        // call prompt
        prompts::read_menu();

        // print existing profile names
        let profiles = data_handler::get_profiles_data();
        let profile_names = data_handler::get_profile_names(&profiles);
        println!("\nProfile Names: {:?}", profile_names);

        // print default profile name
        let default_profile_name = data_handler::get_default_profile_name(&profiles);
        match default_profile_name {
            Ok(name) => println!("Default Profile: {}", name),
            Err(_) => println!("(No Defualt Profile)")
        }

        // ask for which profile the user would like to view data on
        println!("\nPlease enter the name of the profile you'd like to view the info on:");
        prompts::input_symbol();

        let mut desired_profile = String::new();
        io::stdin().read_line(&mut desired_profile).expect("Failed to read your input");
        let desired_profile = desired_profile.trim();

        // get data (of type Profile) for name
        let profile_data = data_handler::get_data_for_profile(&profiles, desired_profile);
        match profile_data {
            Ok(data) => println!("\nHere's the data for profile \"{}\":\n{:?}", desired_profile, data),
            Err(_) => println!("\nThe profile \"{}\" probably doesn't exist! Can't get data.", desired_profile)
        }
    }

    pub fn new_default_menu() {
        // get existing profiles
        let mut profiles = data_handler::get_profiles_data();

        // print existing profiles user may choose to choose from
        let profile_names = data_handler::get_profile_names(&profiles);
        println!("\nHere are the existing profiles: {:?}", profile_names);

        // call prompt
        prompts::new_default_menu();

        // get input from user
        let mut new_default_name = String::new();
        io::stdin().read_line(&mut new_default_name).expect("Failed to read user input");
        let new_default_name = new_default_name.trim();

        // get existing profiles
        let mut profiles = data_handler::get_profiles_data();

        // set new default name
        data_handler::set_new_default_profile_name(&mut profiles, new_default_name);

        // write new data to json
        data_handler::write_profiles_data(&profiles);
    }

    pub fn edit_profile_menu() {
        // get existing profiles
        let mut profiles = data_handler::get_profiles_data();

        // print existing profiles user may choose to choose from
        let profile_names = data_handler::get_profile_names(&profiles);
        println!("\nHere are the existing profiles: {:?}", profile_names);

        // call prompt
        prompts::edit_ask_profile();

        // get input from user
        let mut edit_profile_name = String::new();
        io::stdin().read_line(&mut edit_profile_name).expect("Failed to read user input");
        let edit_profile_name = edit_profile_name.trim();

        // check if entered profile exists in data
        let does_exist = data_handler::does_profile_name_exist(&profiles, edit_profile_name);
        if !does_exist { 
            println!("Sorry, the profile \"{}\" doesn't exist. Please enter one that exists, or consider making a new profile of that name", edit_profile_name);
            return;
        }

        // get data for desired profile
        let mut edit_profile = data_handler::get_data_for_profile(&profiles, edit_profile_name).expect("nope");
        
        // ask if user wants to edit run options
        prompts::edit_ask_change_run();

        // get input from user
        let mut change_run = String::new();
        io::stdin().read_line(&mut change_run).expect("Failed to read user input");
        let change_run = change_run.trim();

        if change_run == "y" {
            // run offline
            println!("\nWould you like to run Minecraft offline? (y/N):");
            prompts::input_symbol();
            let mut run_offline_choice = String::new();
            io::stdin().read_line(&mut run_offline_choice).expect("Failed to read user input");
            if run_offline_choice.trim() == "y" {
                edit_profile.run_options.run_offline = true;
            } else {
                edit_profile.run_options.run_offline = false;
            }

            // change name
            println!("\nWould you like to change the name of your player? (y/N):");
            prompts::input_symbol();
            let mut change_name_choice = String::new();
            io::stdin().read_line(&mut change_name_choice).expect("Failed to read user input");
            if change_name_choice.trim() == "y" {
                edit_profile.run_options.change_name = true;
            } else {
                edit_profile.run_options.change_name = false;
            }

            // new name option
            println!("\nWhat would you like your new player name to be (enter text or just enter for nothing):");
            prompts::input_symbol();
            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).expect("Failed to read user input");
            edit_profile.run_options.new_name = new_name.trim().to_string();

            // auto click play option
            println!("\nWould you like to automatically click play when Minecraft is launched? (y/N):");
            prompts::input_symbol();
            let mut auto_click_play = String::new();
            io::stdin().read_line(&mut auto_click_play).expect("Failed to read user input");
            if auto_click_play.trim() == "y" {
                edit_profile.run_options.auto_click_play = true;
            } else {
                edit_profile.run_options.auto_click_play = false;
            }
        }

        // ask if user wants to edit options.txt settings
        prompts::edit_ask_change_options();

        // get input from user
        let mut change_options = String::new();
        io::stdin().read_line(&mut change_options).expect("Failed to read user input");
        let change_options = change_options.trim();

        if change_options == "y" {
            edit_lines_mainloop(&mut edit_profile.options, ":")
        }

        // ask if user wants to edit optionsshaders.txt settings
        prompts::edit_ask_change_optionsshaders();

        // get input from user
        let mut change_optionsshaders = String::new();
        io::stdin().read_line(&mut change_optionsshaders).expect("Failed to read user input");
        let change_optionsshaders = change_optionsshaders.trim();

        if change_optionsshaders == "y" {
            edit_lines_mainloop(&mut edit_profile.optionsshaders, "=")
        }

        println!("\nThere are no more settings to change.");

        // apply new settings to profile and write out data
        profiles.profiles.insert(edit_profile_name.to_string(), edit_profile);
        data_handler::write_profiles_data(&profiles)
    }

    pub fn edit_lines_mainloop(custom_lines: &mut HashMap<String, String>, seperator: &str) {
        // user interactity loop
        loop {
            println!("\nHere are the current custom options saved:");
            for (key, value) in custom_lines.iter() {
                println!("- {}:{}", key, value);
            }

            println!("\nWhat would you like to do? ('add [text]' 'rm [key]', or '/q'");

            let mut edit_command = String::new();
            io::stdin().read_line(&mut edit_command).expect("Failed to read user input");
            let edit_command = edit_command.trim();

            if edit_command.starts_with("add ") {
                let new_text = &edit_command[4..];
                let parts: Vec<&str> = new_text.split(seperator).collect();
                if let Some(key) = parts.get(0) {
                    if let Some(value) = parts.get(1) {
                        custom_lines.insert(key.to_string(), value.to_string());
                    }
                }
            } else if edit_command.starts_with("rm ") {
                let key_to_remove = &edit_command[3..];

                if let Some(_) = custom_lines.remove(key_to_remove) {
                    println!("Success")
                } else {
                    println!("Not a valid key: {}", key_to_remove)
                }
            } else if edit_command == "/q" {
                println!("Exiting edit options");
                break;
            } else {
                println!("Please enter a valid command!")
            }
        }
    }

    pub fn new_profile_menu() {
        // get existing profiles
        let mut profiles = data_handler::get_profiles_data();

        // print existing profiles user may choose to choose from
        let profile_names = data_handler::get_profile_names(&profiles);
        println!("\nHere are the existing profiles: {:?}", profile_names);

        // call prompt
        prompts::new_profile_menu();

        // get input from user
        let mut new_profile_name = String::new();
        io::stdin().read_line(&mut new_profile_name).expect("Failed to read user input");
        let new_profile_name = new_profile_name.trim().to_string();

        // get default profile config
        let default_profile = data_handler::get_default_new_profile();
        println!("{:?}", default_profile);

        // insert new default profile at name of entered info
        profiles.profiles.insert(new_profile_name, default_profile);

        // write new data to json
        data_handler::write_profiles_data(&profiles)
    }

    pub fn remove_profile_menu() {
        // get existing profiles
        let mut profiles = data_handler::get_profiles_data();

        // print existing profiles user may choose to choose from
        let profile_names = data_handler::get_profile_names(&profiles);
        println!("\nHere are the existing profiles: {:?}", profile_names);

        // call prompt
        prompts::remove_profile_menu();

        // get input from user
        let mut remove_profile_name = String::new();
        io::stdin().read_line(&mut remove_profile_name).expect("Failed to read user input");
        let remove_profile_name = remove_profile_name.trim().to_string();

        // try to remove a profile from the hashmap of profiles
        if let Some(removed_profile) = profiles.profiles.remove(&remove_profile_name) {
            println!("Successfully removed profile \"{}\"", remove_profile_name);
        } else {
            println!("Profile \"{}\" might not exist- couldn't remove it.", remove_profile_name);
        }

        data_handler::write_profiles_data(&profiles);
    }

    pub fn write_menu() {
        println!("\nWriting config to Minecraft Files...");

        // get profiles data
        let profiles = data_handler::get_profiles_data();
        let default_name = data_handler::get_default_profile_name(&profiles).expect("Failed to get default profile.");
        let chosen_profile = data_handler::get_data_for_profile(&profiles, &default_name).expect("Failed to get data for default profile");

        file_handler::write_config_to_minecraft(&chosen_profile);
    }

    pub fn run_menu() {
        // get profiles data
        let profiles = data_handler::get_profiles_data();
        let default_name = data_handler::get_default_profile_name(&profiles).expect("Failed to get default profile.");
        let chosen_profile = data_handler::get_data_for_profile(&profiles, &default_name).expect("Failed to get data for default profile");
        
        println!("\nUsing Profile {}...", default_name);

        write_menu();

        println!("\nOpening Minecraft...");

        let run_offline = chosen_profile.run_options.run_offline;
        let auto_click_play = chosen_profile.run_options.auto_click_play;

        ext_app_handler::open_minecraft(run_offline, auto_click_play);
    }
}

pub fn ui_mainloop() {
    // main loop
    loop {
        // variable for user input
        let mut user_input = String::new();

        // print MAIN MENU prompts
        prompts::main_menu();

        // read input from terminal/console
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        // try to convert to number (corrosponding to menu options)
        match user_input.trim().parse::<u32>() {
            // if valid, process selection for main menu
            Ok(option_num) => {
                // break (exit) if main menu returns true
                if menus::process_main_menu_choose(option_num) {
                    break
                }
            },
            // if invalid, give feedback and continue
            Err(_) => println!("\nPlease enter a number only!")
        }
    }
}
