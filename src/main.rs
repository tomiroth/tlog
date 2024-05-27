use std::fs;
use std::path::PathBuf;
use std::path::Path;

use clap::{Parser, Subcommand};
use homedir::get_my_home;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands{ 
    Test { 
        #[arg(short, long)]
        list :bool 
    }, 
}



fn check_create_dir() ->Option<String> {
    if let Ok(Some(home)) = get_my_home() {
        let dir = format!("{}/{}", home.to_str().unwrap(), ".time_tracker");
        let path = Path::new(&dir);
        let dir = path.to_str().expect(&format!("Could not locate directory {}", dir));
        if !path.exists() {
            fs::create_dir(path).expect(&format!("Could not create path {}",dir))
        } 
        Some(dir.to_owned())
    } else {
        None
    }
}
fn main() {
    let cli = Cli::parse();

    let time_tracker_directory = check_create_dir();


    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }


    println!("{:?}",cli.config);

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
}
