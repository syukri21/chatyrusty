use keycloak::KeycloakError;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BaseError {
    pub code: usize,
    pub messages: String,
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

pub struct SignupResult {}

#[derive(Debug)]
pub struct SignupParams {
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
