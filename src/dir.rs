use std::borrow::Borrow;
use std::fs::{self, read, File};
use std::io::Write;
use std::path::Path;

use homedir::get_my_home;

#[derive(Debug)]
pub struct Dir {
    pub projects_file: String,
}

impl Dir {
    pub fn new() -> Self {
        let root = Self::home_dir().unwrap();
        Dir {
            projects_file: format!("{}/{}", root, "projects"),
        }
    }

    //Checks to see if .time_tracker is in the home dir and creates it if not.
    //Returns its location as a string.
    pub fn home_dir() -> Option<String> {
        if let Ok(Some(home)) = get_my_home() {
            let dir = format!("{}/{}", home.to_str().unwrap(), ".time_tracker");
            let path = Path::new(&dir);
            let dir = path
                .to_str()
                .expect(&format!("Could not locate directory {}", dir));
            if !path.exists() {
                fs::create_dir(path).expect(&format!("Could not create path {}", dir))
            }
            Some(dir.to_owned())
        } else {
            None
        }
    }

    pub fn read(file: &str) -> String {
        let path = Path::new(file);

        if path.is_file() {
            let file_contents: String = String::from_utf8_lossy(&read(path).unwrap())
                .parse()
                .unwrap();
            file_contents
        } else {
            Self::write(file, "").expect(&format!("Could not read file {}", file));
            "".to_owned()
        }
    }

    pub fn write(file: &str, data: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        Ok(())
    }

    pub fn read_project_file(&self) -> String {
        Self::read(&self.projects_file.borrow())
    }
}
