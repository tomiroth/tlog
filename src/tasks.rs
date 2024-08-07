use crate::dir::Dir;
use crate::out::task::TaskOut;
use crate::Task;
use chrono::Datelike;
use chrono::{prelude::Local, Duration, NaiveDate, TimeDelta, Weekday};
use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;

pub struct Tasks<'a> {
    pub inner: Vec<Task<'a>>,
}

use std::collections::HashSet;

pub enum ChronoUnit {
    Year,
    Month,
    Week,
    Day,
}

impl Tasks<'_> {
    pub fn new(unit: ChronoUnit, dir: &Dir) -> Option<Tasks> {
        let mut tasks: Vec<Task> = vec![];

        tasks = match unit {
            ChronoUnit::Year => todo!(),
            ChronoUnit::Month => Self::apply_month(dir, tasks, &dir.current_month),
            ChronoUnit::Week => Self::apply_week(dir, tasks, &dir.current_month),
            ChronoUnit::Day => Self::apply_day(dir, tasks, &dir.current_month),
        };

        if !tasks.is_empty() {
            //Make latest tasks at the top of the vec.
            tasks.reverse();
            Some(Tasks { inner: tasks })
        } else {
            None
        }
    }

    // Function to get the first and last dates of the week
    fn first_and_last_dates_of_week(date: NaiveDate) -> (NaiveDate, NaiveDate) {
        let weekday = date.weekday();
        let first_date = date - Duration::days(weekday.num_days_from_monday() as i64);
        let last_date = first_date + Duration::days(6);
        (first_date, last_date)
    }

    fn apply_week<'a>(dir: &'a Dir, mut tasks: Vec<Task<'a>>, month: &'a str) -> Vec<Task<'a>> {
        // Get today's date
        let today = Local::today().naive_local();

        // Get the first and last dates of the week
        let (first_date, last_date) = Self::first_and_last_dates_of_week(today);

        let month_path = dir.month_file(&dir.current_year, month);
        let path = Path::new(&month_path);
        if path.exists() {
            let file = File::open(path).unwrap();

            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

            //Gather them into a vec.
            for result in rdr.deserialize() {
                let mut task: Task = result.unwrap();
                if first_date < task.start.date().naive_local()
                    && last_date >= task.start.date().naive_local()
                {
                    task.set_dir(dir);
                    task.current = false;
                    tasks.push(task);
                }
            }
        }

        tasks
    }

    fn apply_day<'a>(dir: &'a Dir, mut tasks: Vec<Task<'a>>, month: &'a str) -> Vec<Task<'a>> {
        //Read task fomr current month file
        let month_path = dir.month_file(&dir.current_year, month);
        let path = Path::new(&month_path);
        if path.exists() {
            let file = File::open(path).unwrap();

            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

            //Gather them into a vec.
            for result in rdr.deserialize() {
                let today = Local::today().naive_local();

                let mut task: Task = result.unwrap();
                if today == task.start.date().naive_local()
                    || today == task.end.unwrap().date().naive_local()
                {
                    task.set_dir(dir);
                    task.current = false;
                    tasks.push(task);
                }
            }
        }
        tasks
    }

    pub fn apply_month<'a>(
        dir: &'a Dir,
        mut tasks: Vec<Task<'a>>,
        month: &'a str,
    ) -> Vec<Task<'a>> {
        //Read task fomr current month file
        let month_path = dir.month_file(&dir.current_year, month);
        let path = Path::new(&month_path);
        if path.exists() {
            let file = File::open(path).unwrap();

            let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

            //Gather them into a vec.
            for result in rdr.deserialize() {
                let mut task: Task = result.unwrap();
                task.set_dir(dir);
                task.current = false;
                tasks.push(task);
            }
        }
        tasks
    }

    pub fn time_spent(&self) -> i64 {
        self.inner
            .iter()
            .map(|t| {
                if t.end.is_some() {
                    return (t.end.unwrap() - t.start).num_seconds();
                }
                0
            })
            .sum()
    }

    pub fn output_task(&self) {
        self.inner.iter().for_each(|t| {
            TaskOut::current_task(t);
        });
    }

    pub fn get_names(&self) -> Vec<&str> {
        let unique_names: HashSet<_> = self.inner.iter().map(|t| t.name.as_ref()).collect();
        unique_names.into_iter().collect()
    }

    pub fn get_latest_task_by_name(&self, name: &str) -> Option<&Task> {
        self.inner.iter().find(|t| t.name == name)
    }
}
