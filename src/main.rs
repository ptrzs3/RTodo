use clap::{Parser, Subcommand};
use std::path:: PathBuf;
use std::fs;
#[derive(Subcommand)]
enum Operations {
    /// add a todo item
    Add,
    /// edit a todo item
    Edit,
}
impl Operations {
    fn run(self) {
        match self {
            Operations::Edit => println!("Edit"),
            Operations::Add => println!("Add")
        }
    }
}
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Operations>,

    #[arg(short)]
    /// config file
    config: Option<PathBuf>
}
fn main() {
    let cli = Cli::parse();
    if let Some(opea) = cli.command {
        opea.run();
    };
    if let Some(file) = cli.config {
        if let Ok(_) = fs::create_dir(&file) {
            println!("OK");
        } else {
            println!("Failed");
        }
    }
}
