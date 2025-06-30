use std::net::SocketAddr;

use axum::extract::connect_info::MockConnectInfo;
use sqlx::PgPool;
use zerotoprod::{
    AppState,
    configuration::{Settings, get_configurations},
};

pub struct TestApp {
    pub service: axum::routing::RouterIntoService<axum::body::Body>,
    pub db_pool: PgPool,
    pub config: Settings,
}

pub async fn spawn_app() -> TestApp {
    let config = setup_app_configuration();
    let pool = setup_test_db(&config).await;

    let state = AppState {
        db_pool: pool.clone(),
    };

    let app = zerotoprod::app().with_state(state);
    let mock_connection_info = MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 0)));

    let service = app.layer(mock_connection_info).into_service();

    TestApp {
        service,
        db_pool: pool,
        config,
    }
}

fn setup_app_configuration() -> Settings {
    get_configurations().expect("failed to read configuration in test suite")
}

async fn setup_test_db(config: &Settings) -> PgPool {
    let db_connection_str = &config.database.connection_string();

    let pool = PgPool::connect(db_connection_str).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}

pub async fn tear_down_test_db(pool: &PgPool, table_name: String) {
    sqlx::query(format!("truncate table {} cascade", &table_name).as_str())
        .execute(pool)
        .await
        .expect(format!("failed to tear down table: {}", table_name).as_str());
}
