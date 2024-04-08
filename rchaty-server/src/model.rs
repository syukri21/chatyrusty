use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BaseResp<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SignupParams {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResult {}
