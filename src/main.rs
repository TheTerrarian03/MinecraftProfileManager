#![allow(non_snake_case)]
mod ui;
mod path_handler;
mod file_handler;
mod data_handler;
mod ext_app_handler;

fn main() {
    // check files
    match file_handler::validate_files() {
        Ok(_) => { },
        Err(error) => {
            if error == "No Minecraft Path" {
                println!("It looks like the .minecraft folder exists. Please install Minecraft to the default place and then run this program again!");
                panic!("{}", error)
            }
        }
    }

    // run ui loop
    ui::ui_mainloop();
}
