use base64::DecodeError;
use hmac::digest::MacError;
use keycloak::KeycloakError;
use serde::{Deserialize, Serialize};

use crate::EmailVerifiedMessage;

#[derive(Debug)]
pub struct BaseError {
    pub code: usize,
    pub messages: String,
}

impl BaseError {
    pub fn new(code: usize, messages: &str) -> Self {
        Self {
            code,
            messages: messages.to_string(),
        }
    }
}

impl From<tokio::sync::broadcast::error::SendError<EmailVerifiedMessage>> for BaseError {
    fn from(value: tokio::sync::broadcast::error::SendError<EmailVerifiedMessage>) -> Self {
        tracing::debug!("broadcast error: {:?}", value);
        return BaseError {
            code: 500,
            messages: "broadcast error".to_owned(),
        };
    }
}

impl From<MacError> for BaseError {
    fn from(value: MacError) -> Self {
        tracing::debug!("hmac error: {:?}", value);
        return BaseError {
            code: 400,
            messages: "verified signature error".to_owned(),
        };
    }
}

impl From<DecodeError> for BaseError {
    fn from(value: DecodeError) -> Self {
        tracing::debug!("base64 error: {:?}", value);
        return BaseError {
            code: 500,
            messages: "base64 decode error".to_string(),
        };
    }
}

impl From<uuid::Error> for BaseError {
    fn from(value: uuid::Error) -> Self {
        tracing::debug!("uuid error: {:?}", value);
        return BaseError {
            code: 500,
            messages: value.to_string(),
        };
    }
}

impl From<tokio_postgres::Error> for BaseError {
    fn from(value: tokio_postgres::Error) -> Self {
        tracing::debug!("postgres error: {:?}", value);
        return BaseError {
            code: 500,
            messages: value.to_string(),
        };
    }
}

impl From<reqwest::Error> for BaseError {
    fn from(value: reqwest::Error) -> Self {
        tracing::debug!("reqwest error: {:?}", value);
        return BaseError {
            code: 500,
            messages: value.to_string(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KcloakErrorResponse {
    pub error: String,
    pub error_description: String,
}

impl From<keycloak::KeycloakError> for BaseError {
    fn from(value: keycloak::KeycloakError) -> Self {
        match value {
            KeycloakError::ReqwestFailure(_) => {
                return BaseError {
                    code: 500,
                    messages: value.to_string(),
                }
            }
            KeycloakError::HttpFailure { status, body, text } => {
                if let Some(body) = body {
                    let error_message = body.error_message.unwrap_or(text.clone());
                    tracing::error!("{}", error_message);
                    return BaseError {
                        code: usize::from(status),
                        messages: error_message,
                    };
                } else {
                    return BaseError {
                        code: usize::from(status),
                        messages: text,
                    };
                }
            }
        }
    }
}

impl std::fmt::Display for BaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.messages)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupParams {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninParams {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninResult {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenIntrospect {
    pub exp: Option<i64>,
    pub iat: Option<i64>,
    pub jti: Option<String>,
    pub iss: Option<String>,
    pub aud: Option<String>,
    pub sub: Option<String>,
    pub typ: Option<String>,
    pub azp: Option<String>,
    #[serde(rename = "session_state")]
    pub session_state: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "given_name")]
    pub given_name: Option<String>,
    #[serde(rename = "family_name")]
    pub family_name: Option<String>,
    #[serde(rename = "preferred_username")]
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "email_verified")]
    pub email_verified: Option<bool>,
    pub acr: Option<String>,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(rename = "realm_access")]
    pub realm_access: Option<RealmAccess>,
    #[serde(rename = "resource_access")]
    pub resource_access: Option<ResourceAccess>,
    pub scope: Option<String>,
    pub sid: Option<String>,
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    pub username: Option<String>,
    pub active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceAccess {
    pub account: Account,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub roles: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub email_verified: bool,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifiedEmailCallback {
    pub user_id: String,
    pub token: String,
}
