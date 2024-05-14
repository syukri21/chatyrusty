use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    configuration::CoreConfiguration,
    model::{KcloakErrorResponse, SigninParams, Token, TokenIntrospect, UserInfo},
    BaseError,
};

#[derive(Debug, Clone)]
pub struct KcloakClientConfig {
    pub client_id: String,
    client_secret: String,
    pub url: String,
    realm: String,
}

impl From<Arc<CoreConfiguration>> for KcloakClientConfig {
    fn from(config: Arc<CoreConfiguration>) -> Self {
        KcloakClientConfig {
            client_id: config.keycloak_client_id.to_string(),
            client_secret: config.keycloak_client_secret.to_string(),
            url: config.keycloak_url.to_string(),
            realm: config.keycloak_realm.to_string(),
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
    async fn introspect(&self, token: &str) -> Result<TokenIntrospect, BaseError>;
    async fn user_info(&self, token: &str) -> Result<UserInfo, BaseError>;
    async fn revoke_token(&self, token: &str) -> Result<(), BaseError>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<Token, BaseError>;
}

#[async_trait]
impl KcloakClient for KcloakClientImpl {
    async fn token(&self, request: SigninParams) -> Result<Token, BaseError> {
        let path = format!(
            "/realms/{}/protocol/openid-connect/token",
            self.config.realm
        );
        let url = format!("{}{}", self.config.url, path);
        tracing::debug!("request url: {}", url);
        let params = [
            ("grant_type", "password"),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("username", &request.username_or_email),
            ("password", &request.password),
            ("scope", "openid"),
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

    async fn introspect(&self, token: &str) -> Result<TokenIntrospect, BaseError> {
        let path = format!(
            "/realms/{}/protocol/openid-connect/token/introspect",
            self.config.realm
        );
        let url = format!("{}{}", self.config.url, path);
        tracing::debug!("request url: {}", url);
        let token = token.replace("Bearer ", "");
        let params = [
            ("token", &token),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
        ];
        tracing::debug!("request params: {:?}", params);
        let resp = self.req_client.post(url).form(&params).send().await?;
        if resp.status().is_success() {
            return Ok(resp.json::<TokenIntrospect>().await?);
        } else {
            let errresp = resp.json::<KcloakErrorResponse>().await?;
            return Err(BaseError {
                code: 500,
                messages: errresp.error_description,
            });
        }
    }

    async fn user_info(&self, token: &str) -> Result<UserInfo, BaseError> {
        let path = format!(
            "/realms/{}/protocol/openid-connect/userinfo",
            self.config.realm
        );
        let url = format!("{}{}", self.config.url, path);
        tracing::debug!("request url: {}", url);
        let resp = self
            .req_client
            .get(url)
            .header("Authorization", token)
            .send()
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json::<UserInfo>().await?);
        } else {
            return Err(BaseError {
                code: 500,
                messages: "Token is invalid".to_string(),
            });
        }
    }
    async fn revoke_token(&self, token: &str) -> Result<(), BaseError> {
        let path = format!(
            "/realms/{}/protocol/openid-connect/revoke",
            self.config.realm
        );
        let url = format!("{}{}", self.config.url, path);
        tracing::debug!("request url: {}", url);
        let params = [
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("token", &token.to_string()),
            ("token_type_hint", &"access_token".to_string()),
        ];
        let resp = self.req_client.post(url).form(&params).send().await?;

        if resp.status().is_success() {
            return Ok(());
        } else {
            let err = resp.json::<KcloakErrorResponse>().await?;
            return Err(BaseError {
                code: 500,
                messages: err.error_description,
            });
        }
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<Token, BaseError> {
        let path = format!(
            "/realms/{}/protocol/openid-connect/token",
            self.config.realm
        );
        let url = format!("{}{}", self.config.url, path);
        tracing::debug!("request url: {}", url);
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("refresh_token", &refresh_token),
        ];
        tracing::debug!("request params: {:?}", params);
        let resp = self.req_client.post(url).form(&params).send().await?;
        if resp.status().is_success() {
            return Ok(resp.json::<Token>().await?);
        } else {
            let err = resp.json::<KcloakErrorResponse>().await?;
            return Err(BaseError {
                code: 500,
                messages: err.error_description,
            });
        }
    }
}
