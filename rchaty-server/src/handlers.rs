use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Form, Json,
};
use rchaty_core::{
    model::VerifiedEmailCallback, Auth, EmailVerifiedMessage, SigninParams, SignupParams,
};
use rchaty_web::htmx::{Alert, StoreAuthToken, VerifiedEmailChecker, VerifiedEmailSuccess};

use crate::model::BaseResp;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<dyn Auth + Send + Sync>,
}

pub async fn signup<S>(State(service): State<S>, Form(params): Form<SignupParams>) -> Response<Body>
where
    S: Auth + Send + Sync,
{
    let resp = service.signup(params).await;
    match resp {
        Ok(user_id) => return VerifiedEmailChecker::htmx(user_id).into_response(),
        Err(e) => return (StatusCode::BAD_REQUEST, Alert::htmx(e.messages)).into_response(),
    }
}

pub async fn signin<S>(State(service): State<S>, Form(params): Form<SigninParams>) -> Response<Body>
where
    S: Auth + Send + Sync,
{
    let resp = service.signin(params).await;
    match resp {
        // Ok(ok) => return Json(BaseResp::ok(ok)).into_response(),
        Ok(result) => StoreAuthToken::htmx(
            true,
            "/home",
            result.token,
            result.refresh_token,
            result.expires_in,
        )
        .into_response(),
        Err(e) => return (StatusCode::BAD_REQUEST, Alert::htmx(e.messages)).into_response(),
    }
}

pub async fn send_verify_email<S>(
    headers: HeaderMap,
    State(service): State<S>,
) -> Json<BaseResp<String>>
where
    S: Auth + Send + Sync,
{
    let token: &str = headers
        .get("Authorization")
        .map_or_else(|| "", |v| v.to_str().unwrap_or(""));
    let resp = service.send_verify_email(token).await;
    match resp {
        Ok(_) => return Json(BaseResp::ok_none()),
        Err(e) => return Json(BaseResp::err(e)),
    }
}

pub async fn revoke_token<S>(headers: HeaderMap, State(service): State<S>) -> Json<BaseResp<()>>
where
    S: Auth + Send + Sync,
{
    let token: &str = headers
        .get("Authorization")
        .map_or_else(|| "", |v| v.to_str().unwrap_or(""));
    let token = token.replace("Bearer ", "");
    let resp = service.revoke_token(&token).await;
    match resp {
        Ok(_) => return Json(BaseResp::ok_none()),
        Err(e) => return Json(BaseResp::err(e)),
    }
}

pub async fn callback_verify_email<S>(
    Query(params): Query<VerifiedEmailCallback>,
    State(service): State<S>,
) -> Redirect
where
    S: Auth + Send + Sync,
{
    let resp = service
        .callback_verify_email(&params.user_id, &params.token)
        .await;
    let res = service.get_email_channel().send(EmailVerifiedMessage {
        user_id: params.user_id.to_string(),
        message: VerifiedEmailSuccess::htmx(),
    });
    match res {
        Ok(_) => {
            let msg = format!("email verified for user_id: {}", &params.user_id);
            tracing::info!(msg)
        }
        Err(e) => {
            let msg = format!("email verification failed: {:}", e);
            tracing::error!(msg)
        }
    }
    match resp {
        Ok(_) => return Redirect::to("/login"),
        Err(e) => {
            let msg = format!("/error?msg={}", e);
            return Redirect::to(&msg);
        }
    }
}
