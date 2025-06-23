pub mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use routes::{health_check::health_check, not_found, subscriptions::subscribes};
use tokio::net::TcpListener;

pub async fn run() {
    let listener = TcpListener::bind("0.0.0.0:42069").await.unwrap();

    axum::serve(listener, app().into_make_service())
        .await
        .unwrap();
}

pub fn app() -> Router {
    let router = Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribes))
        .fallback(not_found);

    Router::new()
        .nest("/api/v1", router)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
