use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::fs::{self, read, File};
use std::io::Write;
use std::path::Path;

use chrono::prelude::Local;

use chrono::Datelike;
use homedir::get_my_home;

#[derive(Debug)]
pub struct Dir {
    pub config_file: String,
    ///The directory we store all everything.
    pub time_tracker_dir: String,
    ///The file we store defined projects.
    pub projects_file: String,
    ///The current log file. Current log file will be stored/called
    ///[year]/[month]. So for example if we are currently in June,
    ///2024. Task logs will be saved in 2024/6. The file is a CVS
    pub log_file: String,
    ///This stores the current active task.
    pub current_file: String,
    ///This stores the last active task.
    pub last_file: String,
    pub current_month: String,
    pub current_year: String,
}

impl Dir {
    //todo: handle option unwraping in this method.
    pub fn new() -> Self {
        let home_dir = get_my_home().unwrap().unwrap();
        let time_tracker_dir = Self::time_tracker_dir(home_dir.to_str().unwrap());

        let year = Local::now().year().to_string();
        let month = Local::now().month().to_string();

        let year_dir = Self::year_dir(&time_tracker_dir, &year);

        Dir {
            config_file: format!("{}/{}", &time_tracker_dir, "config.toml"),
            projects_file: format!("{}/{}", &time_tracker_dir, "projects"),
            log_file: format!("{}/{}", year_dir, month),
            current_file: format!("{}/{}", time_tracker_dir, "current"),
            last_file: format!("{}/{}", time_tracker_dir, "last"),
            current_year: year,
            current_month: month,
            time_tracker_dir,
        }
    }

    pub fn month_file(&self, year: &str, month: &str) -> String {
        format!("{}/{}/{}", self.time_tracker_dir, year, month)
    }

    //Creates time_tracker home dir if it does not exist.
    fn time_tracker_dir(home: &str) -> String {
        let dir = format!("{}/{}", home, ".time_tracker");
        let path = Path::new(&dir);
        let dir = path
            .to_str()
            .expect(&format!("Could not locate directory {}", dir));
        if !path.exists() {
            fs::create_dir(path).expect(&format!("Could not create path {}", dir))
        }
        dir.to_owned()
    }

    //Creates dir for the current year if it does not exist
    fn year_dir(time_tacker_dir: &str, year: &str) -> String {
        let dir = format!("{}/{}", time_tacker_dir, year);
        let path = Path::new(&dir);
        let dir = path
            .to_str()
            .expect(&format!("Could not locate directory {}", dir));
        if !path.exists() {
            fs::create_dir(path).expect(&format!("Could not create path {}", dir))
        }
        dir.to_owned()
    }

    fn read(file: &str) -> String {
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

    pub fn write_line(file: &str, data: &str) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().append(true).create(true).open(file)?;

        // Write a new line to the file
        writeln!(file, "{}", data)?;

        Ok(())
    }

    pub fn read_project_file(&self) -> String {
        Self::read(&self.projects_file.borrow())
    }

    fn remove_file(&self, file: &str) {
        let test = fs::remove_file(file).unwrap();
    }

    pub fn remove_current_file(&self) {
        self.remove_file(&self.current_file)
    }

    pub fn get_log_file_location(&self) -> String {
        let path = format!(
            "{}/{}/{}",
            self.time_tracker_dir, self.current_year, self.current_month
        );
        path
    }
}
