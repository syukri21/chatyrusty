use std::sync::Arc;

use axum::{extract::State, Json};
use rchaty_core::{Auth, SigninParams, SigninResult};

use crate::model::{BaseResp, SignupReq};

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<dyn Auth + Send + Sync>,
}

pub async fn signup<S>(
    State(service): State<S>,
    Json(params): Json<SignupReq>,
) -> Json<BaseResp<String>>
where
    S: Auth + Send + Sync,
{
    let resp = service
        .signup(rchaty_core::SignupParams {
            first_name: params.first_name,
            last_name: params.last_name,
            username: params.username,
            email: params.email,
            password: params.password,
        })
        .await;

    if let Err(e) = resp {
        return Json(BaseResp {
            status: e.code.to_string(),
            message: e.messages,
            ..Default::default()
        });
    }
    Json(BaseResp::default())
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
        Ok(resp) => {
            return Json(BaseResp {
                status: "200".to_string(),
                message: "ok".to_string(),
                data: Some(resp),
            })
        }
        Err(e) => {
            return Json(BaseResp {
                status: e.code.to_string(),
                message: e.messages,
                data: None,
            })
        }
    }
}
