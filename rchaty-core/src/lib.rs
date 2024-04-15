use async_trait::async_trait;
use kcloak::Kcloak;
use kcloak::KcloakImpl;
use kcloak_client::KcloakClient;
use kcloak_client::KcloakClientImpl;
use keycloak::types::UserRepresentation;
use std::sync::Arc;

pub use crate::model::BaseError;
pub use crate::model::SigninParams;
pub use crate::model::SigninResult;
pub use crate::model::SignupParams;

pub mod configuration;
pub mod kcloak;
pub mod kcloak_client;
mod model;

#[derive(Clone)]
pub struct AuthImpl {
    kcloak: Arc<dyn Kcloak + Send + Sync>,
    kcloak_client: Arc<dyn KcloakClient + Send + Sync>,
}

impl AuthImpl {
    pub fn new(kcloak: KcloakImpl, kcloak_client: KcloakClientImpl) -> Self {
        AuthImpl {
            kcloak: Arc::new(kcloak),
            kcloak_client: Arc::new(kcloak_client),
        }
    }
}

#[async_trait]
pub trait Auth {
    async fn signup(&self, params: SignupParams) -> Result<(), BaseError>;
    async fn signin(&self, params: SigninParams) -> Result<SigninResult, BaseError>;
    async fn send_verify_email(&self, token: &str) -> Result<(), BaseError>;
    async fn revoke_token(&self, token: &str) -> Result<(), BaseError>;
}

#[async_trait]
impl Auth for AuthImpl {
    async fn signup(&self, params: SignupParams) -> Result<(), BaseError> {
        let client = self.kcloak.get_admin().await?;
        tracing::info!("signup params: {:?}", params);
        let email = Some(params.email);
        client
            .realm_users_post(
                &self.kcloak.get_kconfig().realm,
                UserRepresentation {
                    enabled: Some(true),
                    email: email.clone(),
                    first_name: Some(params.first_name),
                    last_name: Some(params.last_name),
                    email_verified: Some(false),
                    username: email,
                    credentials: Some(vec![keycloak::types::CredentialRepresentation {
                        type_: Some("password".to_string()),
                        temporary: Some(false),
                        value: Some(params.password),
                        ..Default::default()
                    }]),
                    groups: Some(vec!["user".to_string()]),
                    realm_roles: Some(vec!["user".to_string()]),
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }

    async fn signin(&self, params: SigninParams) -> Result<SigninResult, BaseError> {
        let token = self.kcloak_client.token(params).await?;
        Ok(SigninResult {
            token: token.access_token,
            refresh_token: token.refresh_token,
            expires_in: token.expires_in,
        })
    }

    async fn send_verify_email(&self, token: &str) -> Result<(), BaseError> {
        let user_info = self.kcloak_client.user_info(token).await?;
        tracing::debug!("user_info: {:?}", user_info);
        if user_info.email_verified {
            return Err(BaseError {
                code: 400,
                messages: "email already verified".to_string(),
            });
        }
        Ok(self.kcloak.send_email_verification(&user_info.sub).await?)
    }

    async fn revoke_token(&self, token: &str) -> Result<(), BaseError> {
        Ok(self.kcloak_client.revoke_token(token).await?)
    }
}
