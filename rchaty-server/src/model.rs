use rchaty_core::BaseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResp<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

pub trait InnerDataTrait {}

impl<T> BaseResp<T> {
    pub fn err(error: BaseError) -> Self {
        Self {
            status: error.code.to_string(),
            message: error.messages,
            data: None,
        }
    }
    pub fn ok_none() -> Self {
        Self {
            status: "200".to_string(),
            message: "ok".to_string(),
            data: None,
        }
    }
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
