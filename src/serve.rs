pub mod io;
use wstd::http::body::Body;
use wstd::http::{Request, Response};

pub async fn handler(request: Request<Body>) -> anyhow::Result<Response<Body>> {
    let path = request.uri().path_and_query().unwrap().as_str();
    println!("serving {path}");
    match path {
        "/" => hello(request).await,
        _ => not_found().await,
    }
}

async fn not_found() -> anyhow::Result<Response<Body>> {
    let mut response = Response::new(Body::from("404 Not Found\n"));
    *response.status_mut() = wstd::http::StatusCode::NOT_FOUND;
    Ok(response)
}

async fn hello(_request: Request<Body>) -> anyhow::Result<Response<Body>> {
    Ok(Response::new(
        "Hello, wasi:net/TcpListener!\n".to_owned().into(),
    ))
}
