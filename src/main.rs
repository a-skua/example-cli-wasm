mod command;
mod gcloud;
mod rt;
mod serve;
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
    #[command(about = "Execute a shell command")]
    Call {
        #[arg(short, long)]
        command: String,
    },
    #[command(about = "Start HTTP server")]
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    #[command(subcommand)]
    Gcloud(gcloud::Resource),
}

#[wstd::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => command::hello(),
        Commands::Get { url } => command::get_url(&url).await?,
        Commands::Call { command } => command::cli_call(&command),
        Commands::Serve { port } => command::serve(port).await?,
        Commands::Gcloud(r) => gcloud::resource(r).await?,
    }
    Ok(())
}
