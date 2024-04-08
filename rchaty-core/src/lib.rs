use std::sync::Arc;

use async_trait::async_trait;
use kcloak::Kcloak;
use kcloak::KcloakImpl;

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
        let client = self.kcloak.get_client();
        let realmname = client.realm_get("chaty").await?.realm;
        tracing::info!("signup params: {:?}", params);
        tracing::info!("realmname: {:?}", realmname);
        Ok(SignupResult {})
    }
}
