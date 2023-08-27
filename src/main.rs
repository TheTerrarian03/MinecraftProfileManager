#![allow(non_snake_case)]
mod ui;
mod path_handler;
mod file_handler;

fn main() {
    // check files
    file_handler::validate_files();

    // run ui loop
    ui::ui_mainloop();
}
