use crate::serve;
use std::net::TcpListener;
use std::process::Command;
use wstd::http;

pub fn hello() {
    println!("Hello, world!");
}

pub async fn get_url(url: &str) -> anyhow::Result<()> {
    let request = http::Request::get(url).body(http::Body::empty())?;

    let response = http::Client::new().send(request).await?;

    let mut body = response.into_body();
    let contents = body.contents().await?;

    println!("Response body: {}", String::from_utf8_lossy(contents));
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

pub async fn serve(port: u16) -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on {}", listener.local_addr()?);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next() {
        let stream = stream?;
        let request = serve::io::parse_request(&stream).await?;
        let response = serve::handler(request).await?;
        serve::io::write_response(&stream, response).await?;
    }
    Ok(())
}
