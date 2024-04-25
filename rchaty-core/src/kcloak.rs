use std::{fmt::Display, sync::Arc};

use async_trait::async_trait;
use keycloak::{types::UserRepresentation, KeycloakAdmin, KeycloakAdminToken};

use crate::{
    configuration::CoreConfiguration,
    util::{hmac::HmacSignatureImpl, signature::Signature},
    BaseError, SignupParams,
};

pub struct KcloakConfig {
    pub url: String,
    pub realm: String,
    pub username: String,
    pub password: String,
    pub client_id: String,
    send_email_verification_redirect_uri: String,
}
impl From<Arc<CoreConfiguration>> for KcloakConfig {
    fn from(value: Arc<CoreConfiguration>) -> Self {
        KcloakConfig {
            url: value.keycloak_url.to_string(),
            realm: value.keycloak_realm.to_string(),
            username: value.keycloak_admin_username.to_string(),
            password: value.keycloak_admin_password.to_string(),
            client_id: value.keycloak_client_id.to_string(),
            send_email_verification_redirect_uri: value
                .app_redircet_send_verify_email_url
                .to_string(),
        }
    }
}

impl Display for KcloakConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "url {0}, realm {1}, username {2}",
            self.url, self.realm, self.username
        )
    }
}

pub struct KcloakImpl {
    pub kconfig: KcloakConfig,
    pub req_client: reqwest::Client,
}

#[async_trait]
pub trait Kcloak {
    async fn get_admin(&self) -> Result<KeycloakAdmin, BaseError>;
    fn get_kconfig(&self) -> &KcloakConfig;
    async fn send_email_verification(&self, user_id: &str) -> Result<(), BaseError>;
    async fn add_user(&self, params: SignupParams) -> Result<UserRepresentation, BaseError>;
    async fn verify_signature(&self, data: &str, signature: &str) -> Result<(), BaseError>;
}

#[async_trait]
impl Kcloak for KcloakImpl {
    async fn get_admin(&self) -> Result<KeycloakAdmin, BaseError> {
        let token = KeycloakAdminToken::acquire(
            &self.kconfig.url,
            &self.kconfig.username,
            &self.kconfig.password,
            &self.get_req_client(),
        )
        .await?;
        Ok(KeycloakAdmin::new(
            &self.kconfig.url,
            token,
            self.get_req_client(),
        ))
    }

    fn get_kconfig(&self) -> &KcloakConfig {
        &self.kconfig
    }

    async fn send_email_verification(&self, user_id: &str) -> Result<(), BaseError> {
        let admin = self.get_admin().await?;
        let client_id = &self.kconfig.client_id;
        let redirect_uri = self.kconfig.send_email_verification_redirect_uri.to_owned();
        let password = &self.kconfig.password;
        let code = HmacSignatureImpl::new(password.to_owned().to_string()).sign(user_id)?;
        let redirect_uri = format!("{}?token={}&user_id={}", redirect_uri, code, user_id);
        let redirect_uri = Some(redirect_uri);
        admin
            .realm_users_with_id_send_verify_email_put(
                &self.kconfig.realm,
                user_id,
                Some(client_id.to_string()),
                redirect_uri,
            )
            .await?;
        Ok(())
    }

    async fn add_user(&self, params: SignupParams) -> Result<UserRepresentation, BaseError> {
        let client = self.get_admin().await?;
        tracing::info!("signup params: {:?}", params);
        let email = Some(params.email);
        client
            .realm_users_post(
                &self.get_kconfig().realm,
                UserRepresentation {
                    enabled: Some(true),
                    email: email.clone(),
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
                    groups: Some(vec!["user".to_string()]),
                    realm_roles: Some(vec!["user".to_string()]),
                    ..Default::default()
                },
            )
            .await?;

        let user = client
            .realm_users_get(
                &self.get_kconfig().realm,
                None,
                email,
                Some(false),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(1),
                None,
                None,
                None,
            )
            .await?
            .get(0)
            .unwrap()
            .to_owned();

        Ok(user)
    }

    async fn verify_signature(&self, data: &str, signature: &str) -> Result<(), BaseError> {
        let password = &self.kconfig.password;
        tracing::info!("data: {}, signature: {}", data, signature);
        HmacSignatureImpl::new(password.to_owned().to_string()).verify(data, signature)
    }
}

impl KcloakImpl {
    pub async fn new(kconfig: KcloakConfig) -> Result<KcloakImpl, Box<dyn std::error::Error>> {
        Ok(KcloakImpl {
            kconfig,
            req_client: reqwest::Client::new(),
        })
    }

    pub fn get_req_client(&self) -> reqwest::Client {
        self.req_client.clone()
    }
}
