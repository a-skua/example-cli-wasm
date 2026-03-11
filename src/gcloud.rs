use clap::Subcommand;

wit_bindgen::generate!({
    world: "app",
    path: "wit",
    generate_all,
});

use gcloud::storage::buckets::list_buckets;

mod storage {
    use clap::Subcommand;

    #[derive(Subcommand)]
    pub enum Buckets {
        List {
            #[arg(long)]
            project: String,
        },
    }
}

#[derive(Subcommand)]
pub enum Storage {
    #[command(subcommand)]
    Buckets(storage::Buckets),
}

#[derive(Subcommand)]
pub enum Resource {
    #[command(subcommand)]
    Storage(Storage),
}

pub fn resource(r: Resource) -> anyhow::Result<()> {
    match r {
        Resource::Storage(Storage::Buckets(storage::Buckets::List { project })) => {
            match list_buckets(&project) {
                Ok(buckets) => {
                    println!("Buckets in project '{project}':");
                    for bucket in &buckets {
                        println!("  - {}", bucket.name);
                    }
                    if buckets.is_empty() {
                        println!("  (none)");
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e:?}");
                }
            }
        }
    }
    Ok(())
}
