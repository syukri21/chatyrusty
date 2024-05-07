use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use rchaty_core::kcloak_client::{KcloakClient, KcloakClientImpl};
use rchaty_web::htmx::RedirectHtmx;

pub async fn auth_htmx_middleware(
    headers: HeaderMap,
    State(state): State<Arc<KcloakClientImpl>>,
    request: Request,
    next: Next,
) -> Response<Body> {
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return RedirectHtmx::htmx("/login").into_response(),
    };

    let token = token.replace("Bearer ", "");
    tracing::info!("token: {:?}", token);
    let introspect = state
        .introspect(&token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED);

    let introspect = match introspect {
        Ok(introspect) => introspect,
        Err(_) => return RedirectHtmx::htmx("/login").into_response(),
    };

    tracing::info!("introspect: {:?}", introspect.name);
    if !introspect.active {
        return RedirectHtmx::htmx("/login").into_response();
    }
    next.run(request).await
}
