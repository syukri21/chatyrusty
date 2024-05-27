use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use rchaty_core::kcloak_client::{KcloakClient, KcloakClientImpl};
use rchaty_web::htmx::RedirectHtmx;

pub async fn parse_auth(jar: &CookieJar) -> Option<String> {
    return parse_auth_header((jar, "authToken")).await;
}

pub async fn parse_ref_auth(jar: &CookieJar) -> Option<String> {
    return parse_auth_header((jar, "refToken")).await;
}

pub async fn parse_auth_header(jar: (&CookieJar, &str)) -> Option<String> {
    tracing::info!("using cookie header");

    let token = jar.0.get(jar.1);
    if token.is_none() {
        return None;
    }

    let token = token.unwrap();
    if !token.value().is_empty() {
        tracing::info!("using cookie token");
        return Some(token.value().to_string());
    }
    None
}

pub async fn auth_htmx_middleware(
    jar: CookieJar,
    State(state): State<Arc<KcloakClientImpl>>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    {
        let token = parse_auth(&jar).await;
        let token = match token {
            Some(token) => token,
            None => return RedirectHtmx::htmx("/login").into_response(),
        };

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
    };

    {
        let ref_token = parse_ref_auth(&jar).await;
        let ref_token = match ref_token {
            Some(ref_token) => ref_token,
            None => return RedirectHtmx::htmx("/login").into_response(),
        };

        let ref_token = state.refresh_token(&ref_token).await;
        tracing::info!("using refresh token");
        match ref_token {
            Ok(ok) => {
                let response = next.run(request).await;
                let new_cookies = jar
                    .add(Cookie::new("authToken", ok.access_token))
                    .add(Cookie::new("refToken", ok.refresh_token));
                (new_cookies, response).into_response()
            }
            Err(err) => {
                tracing::warn!("failed to refresh token err={}", err);
                return RedirectHtmx::htmx("/login").into_response();
            }
        }
    }
}
