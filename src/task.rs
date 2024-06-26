use core::panic;
use std::fs::OpenOptions;
use std::path::Path;

use chrono::prelude::{DateTime, Local};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};

use crate::dir::Dir;
use crate::input;
use crate::out::projects::ProjectsOut;

use crate::projects::Projects;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Task<'a> {
    pub name: String,
    pub ticket_number: Option<String>,
    pub project: String,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
    #[serde(skip)]
    dir: Option<&'a Dir>,
    #[serde(skip)]
    pub current: bool,
}

impl<'a> Task<'a> {
    pub fn new(dir: &'a Dir, projects: &Projects) -> Option<Self> {
        let last_task = Self::from_last(dir);
        println!("{:?}", last_task);

        let default = last_task.as_ref().map(|t| t.name.to_owned());
        println!("{default:?}");
        let name = match input::input("Task name", default) {
            Some(name) => name,
            _ => panic!("Please entry task name!"),
        };

        let default = last_task.as_ref().and_then(|t| t.ticket_number.clone());
        let ticket_number = input::input("Ticket Number", default);

        let default = last_task.as_ref().map(|t| t.project.to_owned());
        let project = Self::set_project(projects, &default);

        let task = Self {
            name,
            ticket_number,
            project,
            start: Local::now(),
            end: None,
            dir: Some(dir),
            current: true,
        };

        let mut wtr = Writer::from_path(&dir.current_file).unwrap();
        let _ = wtr.serialize(&task);

        Some(task)
    }

    pub fn set_dir(&mut self, dir: &'a Dir) {
        self.dir = Some(dir);
    }

    pub fn start(&self) {
        let mut wtr = Writer::from_path(&self.dir.unwrap().current_file).unwrap();
        let _ = wtr.serialize(self);
    }

    pub fn complete(&mut self) {
        self.end = Some(Local::now());
        self.write_last_file();
        self.dir().remove_current_file();
        self.write_to_log_file();
    }

    pub fn dir(&self) -> &Dir {
        self.dir.expect("Dir should exists")
    }

    pub fn write_to_log_file(&self) {
        let log_location = self.dir().get_log_file_location();
        let path = Path::new(&log_location);
        let include_headers = !path.exists();
        let file = self.dir().get_log_file_location();
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file)
            .unwrap();

        let mut wtr = WriterBuilder::new();
        let mut wtr = wtr.has_headers(include_headers).from_writer(file);
        let _ = wtr.serialize(self);
        let _ = wtr.flush();
    }

    pub fn write_last_file(&self) {
        let mut wtr = Writer::from_path(&self.dir().last_file).unwrap();

        let _ = wtr.serialize(self);
    }

    pub fn from_current(dir: &'a Dir) -> Option<Self> {
        if let Ok(mut rdr) = Reader::from_path(&dir.current_file) {
            if let Some(Ok(Some(task))) = rdr.deserialize().next() {
                let task = Task {
                    dir: Some(dir),
                    current: true,
                    ..task
                };
                return Some(task);
            }
        }
        None
    }

    fn from_last(dir: &'a Dir) -> Option<Self> {
        if let Ok(mut rdr) = Reader::from_path(&dir.last_file) {
            if let Some(Ok(Some(task))) = rdr.deserialize().next() {
                let task = Task {
                    dir: Some(dir),
                    ..task
                };
                return Some(task);
            }
        }
        None
    }

    fn set_project(projects: &Projects, default_value: &Option<String>) -> String {
        let mut project = match input::input("Project", default_value.clone()) {
            Some(project) => project,
            _ => {
                println!("Invalide project");
                ProjectsOut::list(projects);
                Self::set_project(projects, default_value)
            }
        };
        if !projects.exists(&project.to_string()) {
            println!("Invalide project \"{}\"", project);
            ProjectsOut::list(projects);
            project = Self::set_project(projects, default_value);
        }
        project
    }

    pub fn time_spent(&self) -> i64 {
        if self.end.is_some() {
            return (self.end.unwrap() - self.start).num_seconds();
        } else if self.end.is_none() && self.current {
            return (Local::now() - self.start).num_seconds();
        }
        0
    }
}
