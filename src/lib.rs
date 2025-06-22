use axum::{
    Form, Router,
    http::{StatusCode, Uri},
    routing::{get, post},
};
use tokio::net::TcpListener;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
struct SubscribeForm {
    email: String,
    name: String,
}

async fn subscribes(Form(subscriber): Form<SubscribeForm>) -> StatusCode {
    dbg!(&subscriber);
    StatusCode::OK
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
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
        .route("/health-check", get(health_check))
        .route("/subscribes", post(subscribes))
        .fallback(not_found);

    Router::new()
        .nest("/api/v1", router)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
