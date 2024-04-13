use async_trait::async_trait;
use dotenvy::var;
use std::sync::Arc;

use crate::{
    model::{KcloakErrorResponse, SigninParams, Token},
    BaseError,
};

#[derive(Debug, Clone)]
pub struct KcloakClientConfig {
    pub client_id: String,
    client_secret: String,
    pub url: String,
}

impl KcloakClientConfig {
    pub fn from_env() -> Self {
        let client_id = var("KEYCLOAK_CLIENT_ID").expect("KEYCLOAK_CLIENT_ID must be set");
        let client_secret =
            var("KEYCLOAK_CLIENT_SECRET").expect("KEYCLOAK_CLIENT_SECRET must be set");
        let url = var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set");
        KcloakClientConfig {
            client_id,
            client_secret,
            url,
        }
    }
}

pub struct KcloakClientImpl {
    pub config: Arc<KcloakClientConfig>,
    pub req_client: reqwest::Client,
}

impl KcloakClientImpl {
    pub fn new(kconfig: KcloakClientConfig) -> Result<KcloakClientImpl, BaseError> {
        Ok(KcloakClientImpl {
            config: Arc::new(kconfig),
            req_client: reqwest::Client::new(),
        })
    }
}

#[async_trait]
pub trait KcloakClient {
    async fn token(&self, request: SigninParams) -> Result<Token, BaseError>;
}

#[async_trait]
impl KcloakClient for KcloakClientImpl {
    async fn token(&self, request: SigninParams) -> Result<Token, BaseError> {
        let path = "/realms/chaty/protocol/openid-connect/token";
        let url = format!("{}{}", self.config.url, path);
        tracing::debug!("request url: {}", url);
        let params = [
            ("grant_type", "password"),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("username", &request.username_or_email),
            ("password", &request.password),
        ];
        tracing::debug!("request params: {:?}", params);
        let resp = self.req_client.post(url).form(&params).send().await?;
        if resp.status().is_success() {
            return Ok(resp.json::<Token>().await?);
        } else {
            let errresp = resp.json::<KcloakErrorResponse>().await?;
            return Err(BaseError {
                code: 500,
                messages: errresp.error_description,
            });
        }
    }
}
