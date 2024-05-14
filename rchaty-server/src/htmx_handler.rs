use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use rchaty_core::kcloak_client::{KcloakClient, KcloakClientImpl};
use rchaty_web::htmx::{RedirectHtmx, StoreAuthToken};

pub async fn check_auth() -> Response<Body> {
    ("ok").into_response()
}

pub async fn refresh_token(
    headers: HeaderMap,
    State(state): State<Arc<KcloakClientImpl>>,
) -> Response<Body> {
    let token = headers.get("X-Refresh-Token");

    let token = match token {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return RedirectHtmx::htmx("/login").into_response(),
    };

    let token = state.refresh_token(&token).await;
    match token {
        Ok(ok) => {
            return StoreAuthToken::htmx(
                true,
                "/home",
                ok.access_token,
                ok.refresh_token,
                ok.expires_in,
            )
            .into_response();
        }
        Err(_) => return RedirectHtmx::htmx("/login").into_response(),
    };
}
