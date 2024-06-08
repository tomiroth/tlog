use crate::dir::Dir;
use crate::Task;
use csv::ReaderBuilder;
use std::fs::File;

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
    pub fn new(unit: ChronoUnit, dir: &Dir) -> Tasks {
        let mut tasks: Vec<Task> = vec![];

        tasks = match unit {
            ChronoUnit::Year => todo!(),
            ChronoUnit::Month => Self::apply_month(dir, tasks, &dir.current_month),
            ChronoUnit::Week => todo!(),
            ChronoUnit::Day => todo!(),
        };

        //Make latest tasks at the top of the vec.
        tasks.reverse();

        Tasks { inner: tasks }
    }

    pub fn apply_month<'a>(
        dir: &'a Dir,
        mut tasks: Vec<Task<'a>>,
        month: &'a str,
    ) -> Vec<Task<'a>> {
        //Read task fomr current month file
        let file = File::open(dir.month_file(&dir.current_year, &month)).unwrap();
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

        //Gather them into a vec.
        for result in rdr.deserialize() {
            let mut task: Task = result.unwrap();
            task.set_dir(dir);
            tasks.push(task);
        }

        tasks
    }

    pub fn get_names(&self) -> Vec<&str> {
        let unique_names: HashSet<_> = self.inner.iter().map(|t| t.name.as_ref()).collect();
        unique_names.into_iter().collect()
    }

    pub fn get_latest_task_by_name(&self, name: &str) -> Option<&Task> {
        self.inner.iter().find(|t| t.name == name)
    }
}
