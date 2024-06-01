mod dir;
mod projects;

use std::io;
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

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Projects(Projects),
}

#[derive(Subcommand)]
enum Projects {
    List,
    Add { name: String },
    Delete { name: String },
}

#[derive(Parser)]
struct ProjectAdd {
    name: Option<String>,
}

fn confirm(text: String) -> bool {
    let mut input = String::new();
    println!("{} (y):", text);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes" | "")
}

fn main() {
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
                    if confirm(format!("Are you sure you wish to delete project {}", name)) {
                        projects.delete(name);
                        println!("Project {name:?} deleted.")
                    }
                } else {
                    println!("Project {name:?} does not exist.")
                }
            }
        },
    }
}
