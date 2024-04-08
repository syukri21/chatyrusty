use std::{env::var, fmt::Display};

use dotenvy::dotenv;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};

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
    pub client: KeycloakAdmin,
}

pub trait Kcloak {
    fn get_client(&self) -> &KeycloakAdmin;
}

impl Kcloak for KcloakImpl {
    fn get_client(&self) -> &KeycloakAdmin {
        &self.client
    }
}

impl KcloakImpl {
    pub async fn new(kconfig: KcloakConfig) -> Self {
        let client = reqwest::Client::new();
        let kcloak_client_token = KeycloakAdminToken::acquire(
            &kconfig.url,
            &kconfig.username,
            &kconfig.password,
            &client,
        )
        .await
        .expect("Error acquire token");
        let kcloak_client = KeycloakAdmin::new(&kconfig.url, kcloak_client_token, client);
        KcloakImpl {
            kconfig,
            client: kcloak_client,
        }
    }
}
