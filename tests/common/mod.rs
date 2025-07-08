use std::net::SocketAddr;

use axum::extract::connect_info::MockConnectInfo;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zerotoprod::{
    AppState,
    configuration::{DatabaseSetting, Settings, get_configurations},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TestApp {
    pub service: axum::routing::RouterIntoService<axum::body::Body>,
    pub db_pool: PgPool,
    pub config: Settings,
}

impl TestApp {
    async fn terminate_db(&self) {
        println!(
            "cleaning up database: {}",
            self.config.database.database_name
        );

        self.db_pool.close().await;

        let mut connection = PgConnection::connect(&self.config.database.connection_string())
            .await
            .expect("failed to connect to Postgres");

        // force to drop all active connections to database
        connection
            .execute(
                format!(
                    r#"
                    SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE pg_stat_activity.datname = '{}'
                    AND pid <> pg_backend_pid()
                    "#,
                    self.config.database.database_name
                )
                .as_str(),
            )
            .await
            .expect("failed to terminate current connections to test database");

        connection
            .execute(format!(r#"drop database "{}";"#, self.config.database.database_name).as_str())
            .await
            .expect("failed to delete database");

        println!("database cleand up successfully");
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.terminate_db();
    }
}

pub async fn spawn_app() -> TestApp {
    let mut config = setup_app_configuration();

    config.database.database_name = Uuid::new_v4().to_string();
    println!("database name: {}", &config.database.database_name);

    let pool = setup_test_db(&config.database).await;

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

async fn setup_test_db(config: &DatabaseSetting) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("failed to connect to Postgres");

    connection
        .execute(format!(r#"create database "{}";"#, config.database_name).as_str())
        .await
        .expect("failed to create database");

    let db_connection_str = &config.connection_string();

    let pool = PgPool::connect(db_connection_str).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}

#[allow(dead_code)]
pub async fn tear_down_test_db(pool: &PgPool, table_name: String) {
    sqlx::query(format!(r#"truncate table "{}" cascade;"#, &table_name).as_str())
        .execute(pool)
        .await
        .expect(format!("failed to tear down table: {}", table_name).as_str());
}
