mod command;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => command::hello(),
    }
}
