use axum::{body::Body, http::Request};
use common::{spawn_app, tear_down_test_db};
use reqwest::StatusCode;
use sqlx::{Connection, PgConnection};
use tower::{Service, ServiceExt};

mod common;

#[sqlx::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let mut test_app = spawn_app().await;
    let app = &mut test_app.service;

    let mut connection = PgConnection::connect(&test_app.config.database.connection_string())
        .await
        .expect("failed to connect to Postgres");

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

    let saved = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

    tear_down_test_db(&test_app.db_pool, "subscriptions".into()).await;
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let mut test_app = spawn_app().await;
    let app = &mut test_app.service;

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

    tear_down_test_db(&test_app.db_pool, "subscriptions".into()).await;
}
