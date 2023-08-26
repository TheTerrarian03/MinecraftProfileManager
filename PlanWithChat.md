# Planning the Minecraft Profile Manager with ChatGPT

My Rough plan, including things like overall considerations, adaptations of my **old, Python-powered** program, and specifics I may want to consider using when developing my **new, Rust-powered application!**

## Chat Part 1: _Overall considerations:_

- **Dependencies, target platforms, and target architectures:** They will be handled as development goes on
- **Documentation:** This time I plan on having better documentation- between code comments, better laid-out files, and _actually having a `readme.md`_, things should look a lot nicer by the end.
- **Error Handling:** This rewrite, I would certainly like to include more error handling. (I definitely did a terrible job) Though I had planned on it and never got to it last time, I think it's only right that I try harder from the start. From the installation process with making sure data files, expected directories, and anything else is in the right place- or even exists- to handling user input and weird conditions... it'll better (hopefully lol)
- **Configuration:** Suggested by Chat and tried to implement with the python version, this time I would like to better include configuration of how the program works and behaves. This includes paths to files like .minecraft folder and json files.
- **Cross-Playform Compatibility:** Last time, I did a decent job of this by using os.path functions to allow python to automatically handles paths code-independent but interpreter-dependet of platform. I don't know how to do it in Rust, yet, but we'll figure it out. The goal of course is to accomplish the same functionality or better. (I'm sure we can do that though)
- **Build Automation:** Chat suggested multiple times that we create scripts to help with building the app. We'll get to this later in this plan, but I think we will in fact need custom scripts to build. Not sure though!

## Chat Part 2: _Program-Specific Considerations:_

- **File Paths and Handling:** Due to differences in how paths work platform-to-platform, using `std::fs` for dealing with files and `std::path::Path` for navgating, getting parents, and appending paths is a good idea. The equivalent of Python's os.path functions.
- **External Application Interaction:** My program needs to be able to open the Minecraft Launcher 'as' the system so a button can be pressed (either by the user or automatically). Using `std::process::Command` across all platforms and running some platform-specific code is one way to do it. For example(s):

_Windows:_
```rust
Command::new("cmd")
    .args(&["/C", "start", "", file_path])
    .status()
    .expect("Failed to open file with associated program");
```
_macOS:_
```rust
Command::new("open")
    .arg(file_path)
    .status()
    .expect("Failed to open file with associated program");
```
_Linux:_
```rust
Command::new("xdg-open")
    .arg(file_path)
    .status()
    .expect("Failed to open file with associated program");
```
- **JSON Serialization/Deserialization:** My program also needs to read/write to configuration files, holding saved persistent data the user sets, such as profiles and program options. I've chosen to use JSON files for this. Chat recommends adding `serde = "1.0"` and `serde_json = "1.0"` to the `Cargo.toml`, using `serde::{Deserialize, Serialize}`, `std::fs::{File, OpenOptions}`, and `std::io::{Read, Write}` modules, then using serde attributes to define data structures which will represent the data read/written to the files. Examples given:

_"Person" Struct:_
```rust
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}
```
_Reading JSON data:_
```rust
fn read_json_file(file_path: &str) -> Result<Person, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    let person: Person = serde_json::from_reader(reader)?;

    Ok(person)
}
```
_Writing JSON data:_
```rust
fn write_json_file(file_path: &str, person: &Person) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    serde_json::to_writer(file, person)?;

    Ok(())
}
```
- **Configuration File Location:** What I never had, even though I tried so hard to do last time, was set up a simple place for configurations files to exist. I started to get to the point where I am now, where I know what I should use platform-specific paths and places instead of just the root folder of the executable. Chat recommends, reasonably, that I use ` ~/.config/your_app_name` for **mac & linux** and `%appdata%` for **windows**. How do I implement that gracefully? Dunno. The stuff it recommends furthur goes over my head. But I know where we need to go, it's just a matter of getting there through learning.
- **Cross-Platform Testing:** Needa do more.
- **UI/UX Considerations:** Chat recommends someting interesting here. While before I was just designing whatever worked, they're suggesting that I consult **Windows Design Principles** and **macOS's Human Interface Guidelines** for info on how to properly design a UI. I definitely need to do more research here.
- **User Permissions:** Chat has mentioned, for this category, that we consider handling permissions gracefully, for example when we would need to turn on/off wifi elevated permissions are needed. Chat says to _"Handle permissions gracefully and inform users when such actions are required."_

## Chat Part 3: _Feature-Specific Considerations:_
- **UI Packages:** Chat recommends the following UI packages to consider using:
-- `GTK (GIMP Toolkit)`: well optimized, cross-platform, associated with linux
-- `Qt`: great performance, hard to implement, but native look
-- `Druid`: terrible
-- `eframe/egui`: a nice-looking library, recommended by some Youtubers (Here's a link: [Building a GUI app in Rust](https://www.youtube.com/watch?v=NtUkr_z7l84))
-- _Native Platform Libraries:_ `Objective-C bindings` for _macOS_, `WinAPI bindings` for _Windows_, and `Xlib, xcb, etc` libraries for _Linux_
- **Find Image (Button) On Screen:**
- **Accessing Mouse/Keyboard:**
- **Running .bat files:**
- **Running .sh files:**

A picture of a banana:
![banana](https://images.everydayhealth.com/images/diet-nutrition/all-about-bananas-nutrition-facts-health-benefits-recipes-and-more-rm-722x406.jpg)

Website used for markdown: [Dillinger.io](https://dillinger.io/)

## Enjoy!
## -Logan Meyers :D
## 08252023