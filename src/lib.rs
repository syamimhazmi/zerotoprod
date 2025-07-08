pub mod configuration;
pub mod routes;
pub mod startup;

use std::time::Duration;

use axum::{
    Router,
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
};
use configuration::get_configurations;
use routes::{health_check::health_check, not_found, subscriptions::subscribes};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub async fn run() {
    let config = get_configurations().expect("failed to read configuration");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.application_port))
        .await
        .unwrap();

    let db_connection_str = &config.database.connection_string();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(db_connection_str)
        .await
        .expect("can't connect to database");

    let state = AppState { db_pool };

    let app = app().with_state(state);

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub fn app() -> Router<AppState> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribes))
        .fallback(not_found);

    Router::new().nest("/api/v1", router).layer(
        tower_http::trace::TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        }),
    )
}
