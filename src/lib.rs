mod command;
mod gcloud;
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
    #[command(subcommand)]
    Gcloud(gcloud::Resource),
}

pub struct App;

impl wasip2::exports::cli::run::Guest for App {
    fn run() -> Result<(), ()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Hello => command::hello(),
            Commands::Get { url } => {
                wstd::runtime::block_on(async {
                    command::get_url(&url).await.unwrap();
                });
            }
            Commands::Call { command } => command::cli_call(&command),
            Commands::Gcloud(r) => gcloud::resource(r).map_err(|_| ())?,
        }
        Ok(())
    }
}

impl wasip2::exports::http::incoming_handler::Guest for App {
    fn handle(
        request: wasip2::http::types::IncomingRequest,
        response_out: wasip2::http::types::ResponseOutparam,
    ) {
        wstd::runtime::block_on(async {
            let request = wstd::http::request::try_from_incoming(request).unwrap();
            let response = wstd_axum::serve(request, serve::app()).await.unwrap();
            wstd::http::server::Responder::new(response_out)
                .respond(response)
                .await
                .unwrap();
        });
    }
}

wasip2::cli::command::export!(App);
wasip2::http::proxy::export!(App);
