use axum::{Form, debug_handler, extract::State, http::StatusCode};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

use crate::AppState;

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct SubscribeForm {
    email: String,
    name: String,
}

#[debug_handler]
pub async fn subscribes(
    State(state): State<AppState>,
    Form(subscriber): Form<SubscribeForm>,
) -> StatusCode {
    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => {
            log::info!("successfully starting db transaction");

            tx
        }
        Err(err) => {
            log::error!("failed to start db transaction: {:?}", err);

            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let result = sqlx::query!(
        r#"
        insert into subscriptions (uuid, email, name, subscribed_at)
        values($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now(),
    )
    .execute(&mut *tx)
    .await;

    if result.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    match tx.commit().await {
        Ok(_) => {
            log::info!("new subscriber details have been saved");

            StatusCode::OK
        }
        Err(err) => {
            log::error!("failed to execute query: {:?}", err);

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
