mod dir;
mod input;
mod out;
mod projects;
mod task;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use task::Task;

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
    Log {
        #[arg(short, long)]
        current: bool,
        #[arg(short, long)]
        stop: bool,
    },
}

#[derive(Subcommand, Debug)]
enum Projects {
    List,
    Add { name: String },
    Delete { name: String },
}

#[derive(Parser)]
struct ProjectAdd {
    name: Option<String>,
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
        Commands::Log { current, stop } => {
            if *current {
                let task = Task::from_current(&dir);
                TaskOut::current_task(task.expect("Could not create task"));
            } else if *stop {
                let current_task = Task::from_current(&dir);
                if let Some(mut current_task) = current_task {
                    current_task.complete(&dir);
                    TaskOut::current_task(current_task);
                }

                let task = Task::from_current(&dir);
                dir.remove_current_file();
            } else {
                let current_task = Task::from_current(&dir);
                if let Some(mut current_task) = current_task {
                    current_task.complete(&dir)
                }

                let task = task::Task::new(&dir, &projects);
                TaskOut::current_task(task.expect("Could not create task"));
            }
        }
    }
}
