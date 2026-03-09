use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};
use percent_encoding::percent_decode_str;
use std::convert::Infallible;

pub async fn handler(request: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = percent_decode_str(request.uri().path())
        .decode_utf8_lossy()
        .into_owned();
    println!("serving {path}");
    let path: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    match path.as_slice() {
        [] => Ok(hello("hyper")),
        ["greet", name] => Ok(hello(name)),
        _ => Ok(not_found()),
    }
}

fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from("404 Not Found\n")))
        .unwrap()
}

fn hello(name: &str) -> Response<Full<Bytes>> {
    let body = format!("Hello, {name}!\n");
    Response::builder()
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}
