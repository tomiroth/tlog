mod dir;
mod input;
mod projects;
mod task;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
    Log,
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

fn main() -> () {
    let cli = Cli::parse();
    let dir = dir::Dir::new();

    let projects = projects::Projects::new(&dir);

    match &cli.command {
        Commands::Projects(cmd) => match cmd {
            Projects::List => {
                let projects = projects.get();
                println!("Available projects:");
                projects.iter().for_each(|p| println!(" - {}", p));
            }
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
        Commands::Log => {
            let task = task::Task::new();
            println!("{:?}", task);
        }
    }
}
