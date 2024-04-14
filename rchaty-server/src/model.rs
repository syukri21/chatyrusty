use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResp<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl Default for BaseResp<String> {
    fn default() -> Self {
        Self {
            status: "200".to_string(),
            message: "ok".to_string(),
            data: None,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SignupReq {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResult {}
