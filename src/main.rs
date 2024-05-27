mod dir;
mod projects;

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
    command: Commands
}

#[derive(Subcommand)]
enum Commands{ 
    #[command(subcommand)]
    Projects(Projects)
}

#[derive(Subcommand)]
enum Projects{
    List,
    Add,
    Delete
}



fn main() {
    let cli = Cli::parse();
    let dir = dir::Dir::new();


    let projects = projects::Projects::new(&dir);

    match &cli.command {
        Commands::Projects(cmd) => match cmd{
            Projects::List => {
                println!("{:?}", projects.get());
                println!("List Projects")
            }
            Projects::Add => {
                projects.add("testing");
                println!("Add Project")
            }
            Projects::Delete => {
                println!("Delete Project")
            }
        }
    }
}
