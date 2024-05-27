use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use rchaty_core::{
    kcloak_client::{KcloakClient, KcloakClientImpl},
    service::service_contact::{Contact, ContactImpl},
};
use rchaty_web::htmx::{ContactItemHtmx, ContactListHtmx, RedirectHtmx, StoreAuthToken};

use crate::middleware::parse_auth;

pub async fn check_auth() -> Response<Body> {
    ("ok").into_response()
}

pub async fn contact_list(jar: CookieJar, State(state): State<Arc<ContactImpl>>) -> Response<Body> {
    tracing::info!("htmx contact list");

    let token = parse_auth(&jar).await;
    let token = match token {
        Some(token) => token,
        None => return RedirectHtmx::htmx("/login").into_response(),
    };

    let contact_list = state.show_contact_list(&token).await;
    let contact_list = match contact_list {
        Ok(ok) => ok,
        Err(err) => {
            return err.messages.into_response();
        }
    };

    let contact_list: Vec<ContactItemHtmx> = contact_list
        .iter()
        .map(|contact| ContactItemHtmx::new(&contact.user_id, &contact.name))
        .collect();
    ContactListHtmx::htmx(&contact_list).into_response()
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
