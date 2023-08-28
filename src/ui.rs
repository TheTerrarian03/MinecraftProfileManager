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
        println!("3) Clear file");
        println!("4) Quit program");
        input_symbol();
    }

    pub fn read_menu() {
        println!("READ MENU : Here is your text:");
    }

    pub fn write_menu() {
        println!("WRITE MENU : Please enter your lines of text, and type \"/q\" on a new line to quit and write the contents...")
    }
}

mod menus {
    use crate::path_handler;
    use crate::file_handler;
    use super::prompts;
    use std::io;

    // MAIN MENU
    pub fn process_main_menu_choose(option: u32) -> bool {
        match option {
            1 => { read_menu();                  false },
            2 => { write_menu();                 false },
            3 => { clear_menu();                 false },
            4 => { println!("Quitting program"); true  },
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
        // vec (similar to an array/list in Python) to hold lines of user input
        let mut lines: Vec<String> = Vec::new();
        
        // prompt message
        prompts::write_menu();

        // user input loop
        loop {
            // var to hold new info on line
            let mut input = String::new();

            // line prompt
            prompts::input_symbol();

            // get input from stdin
            io::stdin().read_line(&mut input).expect("Failed to read line!");

            // remove trailing newline character
            input = input.trim().to_string();

            // check for quit condition
            if input == "/q" {
                break;
            }

            // add to list of lines
            lines.push(input)
        }

        // try to get path to file
        match path_handler::get_info_file_path() {
            Ok(path) => {
                // try to write content to file
                match file_handler::write_lines_to_info_file(&path, lines) {
                    Ok(_) => println!("Success writing to file"),
                    Err(error) => println!("There was an error writing to file: {}", error)
                }
            },
            Err(error) => println!("There was an error getting the path to the file: {}", error)
        }
    }

    pub fn clear_menu() {
        // call file handler to clear file
        match path_handler::get_info_file_path() {
            Ok(path) => {
                match file_handler::clear_file(&path) {
                    Ok(_) => println!("Success clearing file"),
                    Err(error) => println!("There was an error clearing the file: {}", error)
                }
            },
            Err(error) => println!("There was an error getting the path to the file: {}", error)
        }
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
