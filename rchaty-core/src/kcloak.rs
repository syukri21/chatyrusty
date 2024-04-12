use std::{env::var, fmt::Display};

use async_trait::async_trait;
use dotenvy::dotenv;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};

use crate::BaseError;

pub struct KcloakConfig {
    pub url: String,
    pub realm: String,
    pub username: String,
    pub password: String,
}

impl KcloakConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let url = var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set");
        let username = var("KEYCLOAK_ADMIN").expect("KEYCLOAK_ADMIN must be set");
        let password = var("KEYCLOAK_ADMIN_PASSWORD").expect("KEYCLOAK_ADMIN_PASSWORD must be set");
        let realm = var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set");
        KcloakConfig {
            url,
            realm,
            username,
            password,
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
    async fn get_user_token(&self) -> Result<(), BaseError>;
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

    async fn get_user_token(&self) -> Result<(), BaseError> {
        todo!();
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
