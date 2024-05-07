use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

pub async fn check_auth() -> Response<Body> {
    ("ok").into_response()
}
