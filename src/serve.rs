use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};
use std::convert::Infallible;

pub async fn handler(request: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = request.uri().path();
    println!("serving {path}");
    match path {
        "/" => Ok(hello()),
        _ => Ok(not_found()),
    }
}

fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from("404 Not Found\n")))
        .unwrap()
}

fn hello() -> Response<Full<Bytes>> {
    Response::new(Full::new(Bytes::from("Hello, hyper!\n")))
}
