use axum::Router;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn app() -> Router {
    Router::new()
        .route("/", get(hello_default))
        .route("/greet/{name}", get(hello))
}

async fn hello_default() -> impl IntoResponse {
    "Hello, axum!\n"
}

async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {name}!\n")
}
