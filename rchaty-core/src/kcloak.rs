use std::{fmt::Display, sync::Arc};

use async_trait::async_trait;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};

use crate::{configuration::CoreConfiguration, BaseError};

pub struct KcloakConfig {
    pub url: String,
    pub realm: String,
    pub username: String,
    pub password: String,
    pub client_id: String,
}
impl From<Arc<CoreConfiguration>> for KcloakConfig {
    fn from(value: Arc<CoreConfiguration>) -> Self {
        KcloakConfig {
            url: value.keycloak_url.to_string(),
            realm: value.keycloak_realm.to_string(),
            username: value.keycloak_admin_username.to_string(),
            password: value.keycloak_admin_password.to_string(),
            client_id: value.keycloak_client_id.to_string(),
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
    async fn send_email_verification(&self, email: &str, user_id: &str) -> Result<(), BaseError>;
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

    async fn send_email_verification(&self, _email: &str, user_id: &str) -> Result<(), BaseError> {
        let admin = self.get_admin().await?;
        let actions = vec![format!("VERIFY_EMAIL")];
        let client_id = &self.kconfig.client_id;
        admin
            .realm_users_with_id_execute_actions_email_put(
                &self.kconfig.realm,
                user_id,
                Some(client_id.to_string()),
                Some(600),
                None,
                actions,
            )
            .await?;
        Ok(())
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
