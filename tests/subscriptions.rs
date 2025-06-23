use axum::{body::Body, http::Request};
use common::spawn_app;
use reqwest::StatusCode;
use tower::{Service, ServiceExt};

mod common;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let mut app = spawn_app();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let request = Request::post("/api/v1/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(body))
        .unwrap();

    let response = app
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .expect("failed to subscribe");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let mut app = spawn_app();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let request = Request::post("/api/v1/subscriptions")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(invalid_body))
            .unwrap();

        let response = app
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .expect("failed to execute request");

        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "The API did not fail with 400 bad request when payload was {}",
            error_message
        );
    }
}
