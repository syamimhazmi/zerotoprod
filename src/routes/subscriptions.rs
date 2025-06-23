use axum::{Form, http::StatusCode};

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct SubscribeForm {
    email: String,
    name: String,
}

pub async fn subscribes(Form(subscriber): Form<SubscribeForm>) -> StatusCode {
    dbg!(&subscriber);
    StatusCode::OK
}
