use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use rchaty_core::kcloak_client::{KcloakClient, KcloakClientImpl};
use rchaty_web::htmx::RedirectHtmx;

pub async fn parse_auth(headers: &HeaderMap, jar: &CookieJar) -> Option<String> {
    return parse_auth_header((headers, "Authorization"), (jar, "authToken")).await;
}

#[warn(dead_code)]
pub async fn parse_ref_auth(headers: &HeaderMap, jar: &CookieJar) -> Option<String> {
    return parse_auth_header((headers, "X-Refresh-Token"), (jar, "refToken")).await;
}

pub async fn parse_auth_header(
    headers: (&HeaderMap, &str),
    jar: (&CookieJar, &str),
) -> Option<String> {
    tracing::info!("Parse cookie token type {}", headers.1);
    tracing::info!("Parse header token type {}", jar.1);

    let token = jar.0.get(jar.1);
    if token.is_none() {
        return None;
    }

    let token = token.unwrap();
    if !token.value().is_empty() {
        tracing::info!("using cookie token");
        return Some(token.value().to_string());
    }

    let token = headers.0.get(headers.1);
    let token = match token {
        Some(token) => {
            let token = token.to_str().unwrap().to_string();
            let token = token.replace("Bearer ", "");
            tracing::info!("using header token");
            return Some(token);
        }
        None => None,
    };

    token
}

pub async fn auth_htmx_middleware(
    headers: HeaderMap,
    jar: CookieJar,
    State(state): State<Arc<KcloakClientImpl>>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    {
        let token = parse_auth_header((&headers, "Authorization"), (&jar, "authToken")).await;
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
        let ref_token = parse_auth_header((&headers, "X-Refresh-Token"), (&jar, "refToken")).await;
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
