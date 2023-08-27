// ui module
use std::io;


mod prompts {
    use std::io::{self, Write};

    pub fn input_symbol() {
        print!("$ ");
        io::stdout().flush().expect("error")
    }

    pub fn main_menu() {
        println!("MAIN MENU : Please enter an option:");
        println!("1) Read txt file");
        println!("2) Write text to txt file");
        println!("3) Quit program");
        input_symbol();
    }

    pub fn read_menu() {
        println!("READ MENU : Here is your text:");
    }
}

mod menus {
    use crate::path_handler;
    use crate::file_handler;
    use super::prompts;

    // MAIN MENU
    pub fn process_main_menu_choose(option: u32) -> bool {
        match option {
            1 => { read_menu();                  false },
            2 => { println!("Writing file");     false },
            3 => { println!("Quitting program"); true  },
            _ => { println!("Invalid input");    false }
        }
    }

    // READ MENU
    pub fn read_menu() {
        // call prompt
        prompts::read_menu();
        // read and print info
        match path_handler::get_info_file_path() {
            Ok(path) => {
                // get info from file at the path
                match file_handler::read_file(&path) {
                    Ok(content) => println!("File Contents:\n{}", content),
                    Err(error) => println!("Got an error! {error}")
                }
            },
            Err(err) => println!("Got an error: {err}")
        }
    }

    // WRITE MENU
    pub fn write_menu() {
        // ask for input and write to file until QUIT only
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
                // break if main menu returns true
                if menus::process_main_menu_choose(option_num) {
                    break
                }
            },
            // if invalid, give feedback and continue
            Err(_) => println!("Please enter a number only!")
        }
    }
}
