use std::net::SocketAddr;

use axum::extract::{Request, connect_info::MockConnectInfo};
use tower::Service;

pub fn spawn_app()
-> impl Service<Request, Response = axum::response::Response, Error = std::convert::Infallible> {
    let app = zerotoprod::app();
    let mock_connection_info = MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 0)));

    app.layer(mock_connection_info).into_service()
}
