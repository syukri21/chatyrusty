use keycloak::KeycloakError;

#[derive(Debug)]
pub struct BaseError {
    pub code: usize,
    pub messages: String,
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
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct SigninParams {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct SigninResult {
    pub token: String,
    pub refresh_token: String,
}
