use axum::{Form, extract::State, http::StatusCode};

use crate::AppState;

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct SubscribeForm {
    email: String,
    name: String,
}

pub async fn subscribes(
    State(state): State<AppState>,
    Form(subscriber): Form<SubscribeForm>,
) -> StatusCode {
    dbg!(&state, &subscriber);
    StatusCode::OK
}
