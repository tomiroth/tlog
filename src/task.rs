use core::panic;

use chrono::prelude::{DateTime, Local};
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

use crate::dir::Dir;
use crate::input;
use crate::out::projects::ProjectsOut;
use crate::projects::Projects;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    pub name: String,
    pub ticket_number: Option<String>,
    pub project: String,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(dir: &Dir, projects: &Projects) -> Option<Self> {
        let last_task = Self::from_last(dir);

        let default = last_task.as_ref().map(|t| t.name.to_owned());
        let name = match input::input("Task name", default) {
            Some(name) => name,
            _ => panic!("Please entry task name!"),
        };

        let default = last_task
            .as_ref()
            .map(|t| t.ticket_number.to_owned().unwrap());
        let ticket_number = input::input("Ticket Number", default);

        let default = last_task.as_ref().map(|t| t.project.to_owned());
        let project = Self::set_project(projects, &default);

        let task = Self {
            name,
            ticket_number,
            project,
            start: Local::now(),
            end: None,
        };

        let mut wtr = Writer::from_path(&dir.current_file).unwrap();
        let _ = wtr.serialize(&task);

        Some(task)
    }

    pub fn complete(&mut self, dir: &Dir) {
        self.end = Some(Local::now());

        let mut wtr = Writer::from_path(&dir.last_file).unwrap();
        let _ = wtr.serialize(self);
        dir.remove_current_file();
    }

    pub fn from_current(dir: &Dir) -> Option<Self> {
        if let Ok(mut rdr) = Reader::from_path(&dir.current_file) {
            if let Some(Ok(Some(task))) = rdr.deserialize().next() {
                return Some(task);
            }
        }
        None
    }

    fn from_last(dir: &Dir) -> Option<Self> {
        if let Ok(mut rdr) = Reader::from_path(&dir.last_file) {
            if let Some(Ok(Some(task))) = rdr.deserialize().next() {
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
}
