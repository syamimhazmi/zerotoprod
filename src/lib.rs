use axum::{Router, extract::Path, http::StatusCode, routing::get};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}", name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run() {
    let listener = TcpListener::bind("0.0.0.0:42069").await.unwrap();

    axum::serve(listener, app()).await.unwrap();
}

pub fn app() -> Router {
    Router::new()
        .route("/{name}", get(greet))
        .route("/health-check", get(health_check))
}
