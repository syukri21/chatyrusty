use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use futures::StreamExt;
use rchaty_core::kcloak_client::{KcloakClient, KcloakClientImpl};
use rchaty_web::htmx::{RedirectHtmx, StoreAuthToken};

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
    tracing::info!("using auth token");
    let introspect = state
        .introspect(&token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED);

    let introspect = match introspect {
        Ok(introspect) => introspect,
        Err(_) => return RedirectHtmx::htmx("/login").into_response(),
    };

    tracing::info!("introspect: {:?}", introspect.name);
    if introspect.active {
        return next.run(request).await;
    }

    let ref_token = headers.get("X-Refresh-Token");
    let ref_token = match ref_token {
        Some(ref_token) => ref_token.to_str().unwrap().to_string(),
        None => return RedirectHtmx::htmx("/login").into_response(),
    };

    let ref_token = state.refresh_token(&ref_token).await;
    tracing::info!("using refresh token");
    match ref_token {
        Ok(ok) => {
            tracing::info!("ok: {:?}", ok);
            let response = next.run(request).await;
            let body = response.into_body().into_data_stream();

            let store = StoreAuthToken::htmx(
                false,
                "/home",
                ok.access_token,
                ok.refresh_token,
                ok.expires_in,
            )
            .into_response()
            .into_body()
            .into_data_stream();

            // let store_stream = response.body().into_data_stream();
            // let stream = stream.chain(store_stream);
            let babi = body.chain(store);
            Body::from_stream(babi).into_response()
        }
        Err(_) => return RedirectHtmx::htmx("/login").into_response(),
    }
}
