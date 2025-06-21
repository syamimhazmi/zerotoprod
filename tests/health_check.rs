use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{Request, connect_info::MockConnectInfo},
};
use reqwest::StatusCode;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn test_health_check_succeed() {
    let mut app = spawn_app();

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

fn spawn_app()
-> impl Service<Request, Response = axum::response::Response, Error = std::convert::Infallible> {
    let app = zerotoprod::app();
    let mock_connection_info = MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 0)));

    app.layer(mock_connection_info).into_service()
}
