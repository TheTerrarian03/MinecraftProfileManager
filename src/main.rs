#![allow(non_snake_case)]
mod ui;
mod path_handler;
mod file_handler;

fn main() {
    // get path to mc
    let mc_path = path_handler::get_minecraft_folder();
    println!("{:?}", mc_path);

    // check files
    file_handler::validate_files();

    // run ui loop
    ui::ui_mainloop();
}
