use async_trait::async_trait;

pub use crate::model::BaseError;
pub use crate::model::SignupParams;
pub use crate::model::SignupResult;

mod model;

#[derive(Copy, Clone)]
pub struct AuthImpl {}
impl AuthImpl {
    pub fn new() -> Self {
        AuthImpl {}
    }
}

#[async_trait]
pub trait Auth {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError>;
}

#[async_trait]
impl Auth for AuthImpl {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError> {
        tracing::info!("signup params: {:?}", params);
        Ok(SignupResult {})
    }
}
