use std::sync::Arc;

use async_trait::async_trait;
use kcloak::Kcloak;
use kcloak::KcloakImpl;
use keycloak::types::UserRepresentation;

pub use crate::model::BaseError;
pub use crate::model::SignupParams;
pub use crate::model::SignupResult;

pub mod kcloak;
mod model;

#[derive(Clone)]
pub struct AuthImpl {
    kcloak: Arc<dyn Kcloak + Send + Sync>,
}

impl AuthImpl {
    pub fn new(kcloak: KcloakImpl) -> Self {
        AuthImpl {
            kcloak: Arc::new(kcloak),
        }
    }
}

#[async_trait]
pub trait Auth {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError>;
}

#[async_trait]
impl Auth for AuthImpl {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError> {
        let client = self.kcloak.get_client().await?;
        tracing::info!("signup params: {:?}", params);
        client
            .realm_users_post(
                &self.kcloak.get_kconfig().realm,
                UserRepresentation {
                    enabled: Some(true),
                    email: Some(params.email),
                    first_name: Some(params.first_name),
                    last_name: Some(params.last_name),
                    email_verified: Some(false),
                    username: Some(params.username),
                    credentials: Some(vec![keycloak::types::CredentialRepresentation {
                        type_: Some("password".to_string()),
                        temporary: Some(false),
                        value: Some(params.password),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
            )
            .await?;
        Ok(SignupResult {})
    }
}
