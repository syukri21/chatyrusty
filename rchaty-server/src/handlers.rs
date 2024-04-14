use std::sync::Arc;

use axum::{extract::State, http::HeaderMap, Json};
use rchaty_core::{Auth, SigninParams, SigninResult, SignupParams};

use crate::model::BaseResp;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<dyn Auth + Send + Sync>,
}

pub async fn signup<S>(
    State(service): State<S>,
    Json(params): Json<SignupParams>,
) -> Json<BaseResp<String>>
where
    S: Auth + Send + Sync,
{
    let resp = service.signup(params).await;
    match resp {
        Ok(_) => return Json(BaseResp::ok("ok".to_string())),
        Err(e) => return Json(BaseResp::err(e)),
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
        Ok(_) => return Json(BaseResp::ok("ok".to_string())),
        Err(e) => return Json(BaseResp::err(e)),
    }
}
