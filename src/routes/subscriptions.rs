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
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "adding as a new subscriber",
        %request_id,
        subscriber_name = %subscriber.name,
        subscriber_email = %subscriber.email
    );

    let _request_span_guard = request_span.enter();

    tracing::info!(
        "request id: '{}' - saving subscriber info into database",
        request_id
    );

    let mut tx = match state.db_pool.begin().await {
        Ok(tx) => {
            tracing::info!(
                "request id: '{}' - successfully starting db transaction",
                request_id
            );

            tx
        }
        Err(err) => {
            tracing::error!(
                "request id: '{}' - failed to start db transaction: {:?}",
                request_id,
                err
            );

            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let result = sqlx::query!(
        r#"
        insert into subscriptions (uuid, email, name, subscribed_at, created_at, updated_at)
        values($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now(),
        Utc::now(),
        Utc::now()
    )
    .execute(&mut *tx)
    .await;

    if result.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    match tx.commit().await {
        Ok(_) => {
            tracing::info!(
                "request id: '{}' - new subscriber details have been saved",
                request_id
            );

            StatusCode::OK
        }
        Err(err) => {
            tracing::error!(
                "request id: '{}' - failed to execute query: {:?}",
                request_id,
                err
            );

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
