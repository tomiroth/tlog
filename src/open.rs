use std::path::Path;
pub use std::process::Command;

pub fn open_file_in_editor(command: &Option<String>, filename: &str) {
    let command = match command {
        Some(command) => command,
        None => {
            if cfg!(target_os = "windows") {
                "start"
            } else if cfg!(target_os = "macos") {
                "open"
            } else {
                "xdg-open"
            }
        }
    };

    println!("{:?}", filename);
    // Check if the file exists, create it if not
    if Path::new(filename).exists() {
        let _status = Command::new(command)
            .arg(filename)
            .status()
            .expect("Failed to open file with default editor");
    }
}
