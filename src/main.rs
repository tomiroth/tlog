mod config;
mod dir;
mod input;
mod open;
mod out;
mod projects;
mod task;
mod tasks;

use chrono::Duration;
use task::Task;
use tasks::{ChronoUnit, Tasks};

use clap::{Parser, Subcommand};
use open::open_file_in_editor;

use crate::out::{projects::ProjectsOut, task::pretty_duration, task::TaskOut};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[arg(long)]
    ///Where would you like to save the data for the time tracker.
    data_dir: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///List/Add/remove projects
    #[command(subcommand)]
    Projects(Projects),
    ///Start/Stop task logging. You can only log one task at a time.
    #[command(subcommand)]
    Task(TaskCmd),
    ///Open the latest log file in your default editor or specified
    ///editor in config.toml
    Open,
    ///See how much time you have logged today
    Logged,
}

#[derive(Subcommand, Debug)]
enum Projects {
    ///List current available tasks
    List,
    ///Add a projects
    Add { name: String },
    ///Delete a project
    Delete { name: String },
}

#[derive(Subcommand, Debug)]
enum TaskCmd {
    ///Start a task. -l to select a task you previously have logged time to.
    ///Starting a task while one is running will stop the current task.
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
    ///Stop current task.
    Stop,
    ///Output time spent on current task
    Current,
}

#[derive(Parser)]
struct ProjectAdd {
    name: Option<String>,
}

#[cfg(debug_assertions)]
fn is_debug() -> bool {
    true
}

#[cfg(not(debug_assertions))]
fn is_debug() -> bool {
    false
}

fn complete_current_task(dir: &dir::Dir) {
    let current_task = Task::from_current(dir);
    if let Some(mut current_task) = current_task {
        current_task.complete()
    }
}
fn main() {
    let cli = Cli::parse();

    let data_dir = cli.data_dir;
    //Putting this in as a safe gaurd so i don't over write my
    //time tracker data when testing.
    if is_debug() && data_dir.is_none() {
        panic!("Please specify which directory to save tracking files while devoloping. You can specify the directory using --data-dir")
    }

    let dir = dir::Dir::new(data_dir);
    let config = config::Config::new(&dir.config_file);
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
                    if let Some(tasks) = tasks {
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
                        println!("No existing tasks to select from.");
                    }
                } else {
                    complete_current_task(&dir);

                    let task = task::Task::new(&dir, &projects);
                    TaskOut::current_task(&task.expect("Could not create task"));
                }
            }
            TaskCmd::Stop => {
                let current_task = Task::from_current(&dir);
                if let Some(mut current_task) = current_task {
                    current_task.complete();
                    TaskOut::current_task(&current_task);
                }

                let _task = Task::from_current(&dir);
            }
            TaskCmd::Current => {
                let task = Task::from_current(&dir);
                TaskOut::current_task(&task.expect("Could not create task"));
            }
        },
        Commands::Open => {
            open_file_in_editor(&config.editor, &dir.log_file);
        }
        Commands::Logged => {
            let tasks = Tasks::new(ChronoUnit::Day, &dir).unwrap();
            tasks.output_task();
            let time_spent = tasks.time_spent();

            println!();
            println!();
            println!("Current Task:");
            let current_task = Task::from_current(&dir).unwrap();
            TaskOut::current_task(&current_task);

            let dur = Duration::new(time_spent + current_task.time_spent(), 0).unwrap();
            println!("Time spent today: {}", pretty_duration(dur));
        }
    }
}
