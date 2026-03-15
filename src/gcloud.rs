use clap::Subcommand;

wit_bindgen::generate!({
    world: "app",
    path: "wit",
    generate_all,
    with: {
        "wasi:io/error@0.2.9": wasip2::io::error,
        "wasi:io/poll@0.2.9": wasip2::io::poll,
        "wasi:io/streams@0.2.9": wasip2::io::streams,
        "wasi:cli/environment@0.2.9": wasip2::cli::environment,
        "wasi:cli/exit@0.2.9": wasip2::cli::exit,
        "wasi:cli/stderr@0.2.9": wasip2::cli::stderr,
        "wasi:clocks/monotonic-clock@0.2.9": wasip2::clocks::monotonic_clock,
        "wasi:random/insecure-seed@0.2.9": wasip2::random::insecure_seed,
        "wasi:http/types@0.2.9": wasip2::http::types,
        "wasi:http/outgoing-handler@0.2.9": wasip2::http::outgoing_handler,
        "wasi:cli/run@0.2.9": wasip2::exports::wasi::cli::run,
        "wasi:http/incoming-handler@0.2.9": wasip2::exports::wasi::http::incoming_handler,
    },
});

use gcloud::auth::token_source::get_token;
use gcloud::storage::buckets::{create_bucket, delete_bucket, get_bucket, list_buckets};

mod storage {
    use clap::Subcommand;

    #[derive(Subcommand)]
    pub enum Buckets {
        List {
            #[arg(long)]
            project: String,
        },
        Get {
            #[arg(long)]
            name: String,
        },
        Create {
            #[arg(long)]
            project: String,
            #[arg(long)]
            name: String,
        },
        Delete {
            #[arg(long)]
            name: String,
        },
    }
}

mod auth {
    use clap::Subcommand;

    #[derive(Subcommand)]
    pub enum Token {
        Get {
            #[arg(long)]
            scopes: Vec<String>,
        },
    }
}

#[derive(Subcommand)]
pub enum Storage {
    #[command(subcommand)]
    Buckets(storage::Buckets),
}

#[derive(Subcommand)]
pub enum Auth {
    #[command(subcommand)]
    Token(auth::Token),
}

#[derive(Subcommand)]
pub enum Resource {
    #[command(subcommand)]
    Storage(Storage),
    #[command(subcommand)]
    Auth(Auth),
}

fn mask_token(token: &str) -> String {
    if token.len() <= 4 {
        return "***".to_string();
    }
    format!("{}***", &token[..4])
}

fn print_bucket(bucket: &gcloud::storage::types::Bucket) {
    println!("  name: {}", bucket.name);
    if let Some(location) = &bucket.location {
        println!("  location: {location}");
    }
    if let Some(storage_class) = &bucket.storage_class {
        println!("  storage-class: {storage_class}");
    }
    if let Some(time_created) = &bucket.time_created {
        println!("  time-created: {time_created}");
    }
}

pub fn resource(r: Resource) -> anyhow::Result<()> {
    match r {
        Resource::Auth(Auth::Token(auth::Token::Get { scopes })) => match get_token(&scopes) {
            Ok(token) => {
                let masked = mask_token(&token.access_token);
                println!("Token:");
                println!("  access-token: {masked}");
                println!("  token-type: {}", token.token_type);
                println!("  expires-in: {}", token.expires_in);
            }
            Err(e) => {
                eprintln!("Error: {e:?}");
            }
        },
        Resource::Storage(Storage::Buckets(cmd)) => match cmd {
            storage::Buckets::List { project } => match list_buckets(&project) {
                Ok(buckets) => {
                    println!("Buckets in project '{project}':");
                    for bucket in &buckets {
                        print_bucket(bucket);
                        println!();
                    }
                    if buckets.is_empty() {
                        println!("  (none)");
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e:?}");
                }
            },
            storage::Buckets::Get { name } => match get_bucket(&name) {
                Ok(bucket) => {
                    println!("Bucket:");
                    print_bucket(&bucket);
                }
                Err(e) => {
                    eprintln!("Error: {e:?}");
                }
            },
            storage::Buckets::Create { project, name } => match create_bucket(&project, &name) {
                Ok(bucket) => {
                    println!("Created bucket:");
                    print_bucket(&bucket);
                }
                Err(e) => {
                    eprintln!("Error: {e:?}");
                }
            },
            storage::Buckets::Delete { name } => match delete_bucket(&name) {
                Ok(()) => {
                    println!("Deleted bucket '{name}'");
                }
                Err(e) => {
                    eprintln!("Error: {e:?}");
                }
            },
        },
    }
    Ok(())
}
