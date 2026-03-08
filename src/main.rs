mod command;
use clap::{Parser, Subcommand};
use std::process;
use anyhow::Result;

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

#[wstd::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => command::hello(),
    }
    Ok(())
}
