use wstd::http::{Body, Client, Request};

pub fn hello() {
    println!("Hello, world!");
}

pub async fn get_url(url: &str) -> anyhow::Result<()> {
    let request = Request::get(url).body(Body::empty())?;

    let response = Client::new().send(request).await?;

    let mut body = response.into_body();
    let contents = body.contents().await?;

    println!("Response body: {}", String::from_utf8_lossy(&contents));
    Ok(())
}
