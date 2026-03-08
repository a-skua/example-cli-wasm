use std::process::Command;
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

/// TODO: Failed to execute command: Error { kind: Unsupported, message: "operation not supported on this platform" }
pub fn cli_call(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!(
            "Command output: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        eprintln!("Command error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
