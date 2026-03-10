use clap::Subcommand;
use wstd::http::{Body, Client, Request};

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

pub async fn resource(r: Resource) -> anyhow::Result<()> {
    match r {
        Resource::Storage(Storage::Buckets(storage::Buckets::List { project })) => {
            list_buckets(&project).await?;
        }
    }
    Ok(())
}

async fn get_access_token() -> anyhow::Result<String> {
    let adc_path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or_else(|_| {
        let home = std::env::var("HOME").expect("HOME is not set");
        format!("{home}/.config/gcloud/application_default_credentials.json")
    });

    let adc_json = std::fs::read_to_string(&adc_path)
        .map_err(|e| anyhow::anyhow!("failed to read ADC file {adc_path}: {e}"))?;

    #[derive(serde::Deserialize)]
    struct Adc {
        client_id: String,
        client_secret: String,
        refresh_token: String,
    }
    let adc: Adc = serde_json::from_str(&adc_json)?;

    let form_body = format!(
        "grant_type=refresh_token&client_id={}&client_secret={}&refresh_token={}",
        percent_encoding::utf8_percent_encode(&adc.client_id, percent_encoding::NON_ALPHANUMERIC),
        percent_encoding::utf8_percent_encode(
            &adc.client_secret,
            percent_encoding::NON_ALPHANUMERIC,
        ),
        percent_encoding::utf8_percent_encode(
            &adc.refresh_token,
            percent_encoding::NON_ALPHANUMERIC,
        ),
    );

    let request = Request::post("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(form_body))?;

    let response = Client::new().send(request).await?;
    let mut body = response.into_body();
    let contents = body.contents().await?;

    #[derive(serde::Deserialize)]
    struct TokenResponse {
        access_token: String,
    }
    let token: TokenResponse = serde_json::from_slice(contents)?;
    Ok(token.access_token)
}

async fn list_buckets(project: &str) -> anyhow::Result<()> {
    let token = get_access_token().await?;

    let url = format!(
        "https://storage.googleapis.com/storage/v1/b?project={}",
        percent_encoding::utf8_percent_encode(project, percent_encoding::NON_ALPHANUMERIC),
    );

    let request = Request::get(&url)
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::empty())?;
    let response = Client::new().send(request).await?;

    let mut body = response.into_body();
    let contents = body.contents().await?;

    println!("{}", String::from_utf8_lossy(contents));
    Ok(())
}
