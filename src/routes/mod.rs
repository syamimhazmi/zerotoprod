use axum::http::{StatusCode, Uri};

pub mod health_check;
pub mod subscriptions;

pub async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Not found for {uri}"))
}
