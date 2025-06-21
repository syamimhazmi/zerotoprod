use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}", name)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn run() {
    let listener = TcpListener::bind("0.0.0.0:42069").await.unwrap();

    axum::serve(listener, app()).await.unwrap();
}

pub fn app() -> Router {
    let router = Router::new()
        .route("/{name}", get(greet))
        .route("/health-check", get(health_check))
        .fallback(not_found);

    Router::new()
        .nest("/api/v1", router)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
