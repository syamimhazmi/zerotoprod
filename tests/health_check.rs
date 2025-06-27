use axum::{body::Body, extract::Request};
use common::spawn_app;
use reqwest::StatusCode;
use sqlx::{Connection, PgConnection};
use tower::{Service, ServiceExt};
use zerotoprod::configuration::get_configurations;

mod common;

#[tokio::test]
async fn test_health_check_succeed() {
    let mut app = spawn_app();
    let config = get_configurations().expect("failed to read configuration.");

    let _connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to Postgres");

    let request = Request::builder()
        .uri("/api/v1/health-check")
        .body(Body::empty())
        .unwrap();

    let response = app
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .expect("failed to call health-check request");

    assert_eq!(response.status(), StatusCode::OK);
}
