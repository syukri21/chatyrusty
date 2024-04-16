use crate::db::repository::DBImpl;
use crate::db::repository::DB;
use crate::kcloak::Kcloak;
use crate::kcloak::KcloakImpl;
use crate::kcloak_client::KcloakClient;
use crate::kcloak_client::KcloakClientImpl;
use crate::BaseError;
use crate::SigninParams;
use crate::SigninResult;
use crate::SignupParams;

use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthImpl {
    kcloak: Arc<dyn Kcloak + Send + Sync>,
    kcloak_client: Arc<dyn KcloakClient + Send + Sync>,
    db: Arc<dyn DB + Send + Sync>,
}

impl AuthImpl {
    pub fn new(kcloak: KcloakImpl, kcloak_client: KcloakClientImpl, db: DBImpl) -> Self {
        AuthImpl {
            kcloak: Arc::new(kcloak),
            kcloak_client: Arc::new(kcloak_client),
            db: Arc::new(db),
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
        let user = self.kcloak.add_user(params).await?;
        let user_id = user.id.unwrap();
        let _ = self.kcloak.send_email_verification(&user_id).await;

        // TODO: save user representation to db
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
