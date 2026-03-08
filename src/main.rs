mod command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Prints 'Hello, World!'")]
    Hello,
    #[command(about = "Get content from a URL")]
    Get {
        #[arg(short, long)]
        url: String,
    },
}

#[wstd::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => command::hello(),
        Commands::Get { url } => command::get_url(&url).await?,
    }
    Ok(())
}
