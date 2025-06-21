use axum::{
    Router,
    extract::Path,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}", name)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn not_found(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Not found for {uri}"))
}

pub async fn run() {
    let listener = TcpListener::bind("0.0.0.0:42069").await.unwrap();

    axum::serve(listener, app().into_make_service())
        .await
        .unwrap();
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
