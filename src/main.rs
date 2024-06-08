mod dir;
mod input;
mod out;
mod projects;
mod task;
mod tasks;

use std::path::PathBuf;
use tui;

use clap::{Parser, Subcommand};
use task::Task;
use tasks::{ChronoUnit, Tasks};

use crate::out::{projects::ProjectsOut, task::TaskOut};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand)]
    Projects(Projects),
    #[command(subcommand)]
    Task(TaskCmd),
}

#[derive(Subcommand, Debug)]
enum Projects {
    List,
    Add { name: String },
    Delete { name: String },
}

#[derive(Subcommand, Debug)]
enum TaskCmd {
    Start {
        #[arg(short, long)]
        /// Select tasks from existing list of tasks, by default it will be
        /// tasks you have logged time against in the past. It only list
        /// tasks done this month
        list: bool,
        #[arg(short, long)]
        /// Select task from a markdown file. It will find all lines that
        /// start with:
        /// " - [ ]" and list them as a task.
        mark_down: bool,
        /// Select heading the checked items should come from in markdown
        /// file. It will use all lines up until the next heading.
        #[arg(long)]
        mark_down_heading: bool,
        #[arg(short, long)]
        /// Use what is in your clipboard for the name of the task.
        clip_board: bool,
    },
    Stop,
    Current,
}

#[derive(Parser)]
struct ProjectAdd {
    name: Option<String>,
}

fn complete_current_task(dir: &dir::Dir) {
    let current_task = Task::from_current(&dir);
    if let Some(mut current_task) = current_task {
        current_task.complete()
    }
}
fn main() {
    let cli = Cli::parse();
    let dir = dir::Dir::new();

    let projects = projects::Projects::new(&dir);

    match &cli.command {
        Commands::Projects(cmd) => match cmd {
            Projects::List => ProjectsOut::list(&projects),
            Projects::Add { name } => {
                projects.add(name);
                println!("Added project {name:?}");
            }
            Projects::Delete { name } => {
                if projects.exists(name) {
                    if input::confirm(format!("Are you sure you wish to delete project {}", name)) {
                        projects.delete(name);
                        println!("Project {name:?} deleted.")
                    }
                } else {
                    println!("Project {name:?} does not exist.")
                }
            }
        },
        Commands::Task(cmd) => match cmd {
            TaskCmd::Start { list, .. } => {
                if *list {
                    let tasks = Tasks::new(ChronoUnit::Month, &dir);
                    let task_names = tasks.get_names();
                    let task_name = tui::menu("Select task:", &task_names);
                    let task = tasks.get_latest_task_by_name(task_name);

                    if let Some(task) = task {
                        complete_current_task(&dir);

                        task.start();
                        println!("{:?}", task);
                    } else {
                        todo!();
                    }
                } else {
                    complete_current_task(&dir);

                    let task = task::Task::new(&dir, &projects);
                    TaskOut::current_task(task.expect("Could not create task"));
                }
            }
            TaskCmd::Stop => {
                let current_task = Task::from_current(&dir);
                if let Some(mut current_task) = current_task {
                    current_task.complete();
                    TaskOut::current_task(current_task);
                }

                let _task = Task::from_current(&dir);
            }
            TaskCmd::Current => {
                let task = Task::from_current(&dir);
                TaskOut::current_task(task.expect("Could not create task"));
            }
        },
    }
}
