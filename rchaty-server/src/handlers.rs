use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use rchaty_core::Auth;

use crate::model::{BaseResp, SignupReq};

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<dyn Auth + Send + Sync>,
}

pub async fn signup<S>(
    Query(_params): Query<SignupReq>,
    State(service): State<S>,
) -> Json<BaseResp<String>>
where
    S: Auth + Send + Sync,
{
    let _ = service
        .signup(rchaty_core::SignupParams {
            first_name: "Foo".to_string(),
            last_name: "Bar".to_string(),
            email: "foo@bar".to_string(),
            password: "12345".to_string(),
        })
        .await
        .unwrap();
    Json(BaseResp::default())
}
