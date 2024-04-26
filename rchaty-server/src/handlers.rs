use std::sync::Arc;

use askama::Template;
use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Form, Json,
};
use rchaty_core::{model::VerifiedEmailCallback, Auth, SigninParams, SigninResult, SignupParams};
use rchaty_web::htmx::{Alert, VerifiedEmailChecker};

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
        Ok(_resp) => return VerifiedEmailChecker::htmx().into_response(),
        Err(e) => {
            let alert = Alert::new(e.messages);
            return (StatusCode::BAD_REQUEST, alert.render().unwrap()).into_response();
        }
    }
}

pub async fn signin<S>(
    State(service): State<S>,
    Json(params): Json<SigninParams>,
) -> Json<BaseResp<SigninResult>>
where
    S: Auth + Send + Sync,
{
    let resp = service.signin(params).await;
    match resp {
        Ok(resp) => return Json(BaseResp::ok(resp)),
        Err(e) => return Json(BaseResp::err(e)),
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
    match resp {
        Ok(_) => return Redirect::to("/login"),
        Err(e) => {
            let msg = format!("/error?msg={}", e);
            return Redirect::to(&msg);
        }
    }
}
