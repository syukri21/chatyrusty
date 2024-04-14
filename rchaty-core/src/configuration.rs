use std::sync::Arc;

use dotenvy::{dotenv, var};

#[derive(Debug, Clone)]
pub struct CoreConfiguration {
    pub keycloak_admin_username: Arc<String>,
    pub keycloak_admin_password: Arc<String>,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub database_url: String,
}

impl CoreConfiguration {
    pub fn from_env() -> CoreConfiguration {
        dotenv().ok();
        let keycloak_admin_username: String =
            var("KEYCLOAK_ADMIN_USERNAME").expect("KEYCLOAK_ADMIN_USERNAME must be set");
        let keycloak_admin_password =
            var("KEYCLOAK_ADMIN_PASSWORD").expect("KEYCLOAK_ADMIN_PASSWORD must be set");
        let keycloak_url = var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set");
        let keycloak_realm = var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set");
        let keycloak_client_id = var("KEYCLOAK_CLIENT_ID").expect("KEYCLOAK_CLIENT_ID must be set");
        let keycloak_client_secret =
            var("KEYCLOAK_CLIENT_SECRET").expect("KEYCLOAK_CLIENT_SECRET must be set");

        // database
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

        CoreConfiguration {
            keycloak_admin_username: Arc::new(keycloak_admin_username),
            keycloak_admin_password: Arc::new(keycloak_admin_password),
            keycloak_url,
            keycloak_realm,
            keycloak_client_id,
            keycloak_client_secret,
            database_url,
        }
    }

    pub fn from_env_arc() -> Arc<CoreConfiguration> {
        Arc::new(CoreConfiguration::from_env())
    }
}
