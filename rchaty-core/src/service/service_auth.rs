use crate::db::repository::DB;
use crate::kcloak::Kcloak;
use crate::kcloak::KcloakImpl;
use crate::kcloak_client::KcloakClient;
use crate::kcloak_client::KcloakClientImpl;
use crate::BaseError;
use crate::EmailVerifiedChannel;
use crate::EmailVerifiedChannelImpl;
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
    email_channel: Arc<dyn EmailVerifiedChannel + Send + Sync>,
}

impl AuthImpl {
    pub fn new(
        kcloak: KcloakImpl,
        kcloak_client: Arc<KcloakClientImpl>,
        db: Arc<dyn DB + Send + Sync>,
        email_channel: EmailVerifiedChannelImpl,
    ) -> Self {
        AuthImpl {
            kcloak: Arc::new(kcloak),
            kcloak_client,
            db,
            email_channel: Arc::new(email_channel),
        }
    }
}

#[async_trait]
pub trait Auth {
    async fn signup(&self, params: SignupParams) -> Result<String, BaseError>;
    async fn signin(&self, params: SigninParams) -> Result<SigninResult, BaseError>;
    async fn send_verify_email(&self, token: &str) -> Result<(), BaseError>;
    async fn revoke_token(&self, token: &str) -> Result<(), BaseError>;
    async fn callback_verify_email(&self, user_id: &str, token: &str) -> Result<(), BaseError>;
    fn get_email_channel(&self) -> Arc<dyn EmailVerifiedChannel + Send + Sync>;
}

#[async_trait]
impl Auth for AuthImpl {
    async fn signup(&self, params: SignupParams) -> Result<String, BaseError> {
        let user = self.kcloak.add_user(params).await?;
        self.db.save_user(&user).await?;

        // send email verification to user after signup
        {
            let user_id = Arc::new(user.id.to_owned().unwrap());
            let kcloak = self.kcloak.clone();
            tokio::spawn(async move {
                let user_id = user_id.as_ref();
                tracing::info!("send email verification for user_id: {:?}", user_id);
                kcloak.send_email_verification(user_id).await.unwrap();
            });
        }

        Ok(user.id.unwrap())
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

    async fn callback_verify_email(&self, user_id: &str, token: &str) -> Result<(), BaseError> {
        tracing::info!("user_id: {:?}, token: {:?}", user_id, token);
        self.kcloak.verify_signature(user_id, token).await?;
        self.db.update_verified_email(user_id).await?;
        Ok(())
    }

    fn get_email_channel(&self) -> Arc<dyn EmailVerifiedChannel + Send + Sync> {
        Arc::clone(&self.email_channel)
    }
}
