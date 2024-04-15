use std::sync::Arc;

use dotenvy::{dotenv, var};

#[derive(Debug, Clone)]
pub struct CoreConfiguration {
    pub app_redircet_send_verify_email_url: String,
    pub keycloak_admin_username: Arc<String>,
    pub keycloak_admin_password: Arc<String>,
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub database_host: String,
    pub database_port: u16,
    pub database_user: String,
    pub database_password: String,
    pub database_name: String,
}

impl CoreConfiguration {
    pub fn from_env() -> CoreConfiguration {
        dotenv().ok();

        // app
        let app_redircet_send_verify_email_url = var("APP_REDIRECT_SEND_VERIFY_EMAIL_URL")
            .expect("APP_REDIRECT_SEND_VERIFY_EMAIL_URL must be set");

        // kcloak
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
        let database_host = var("DATABASE_HOST").expect("DATABASE_HOST must be set");
        let database_port = var("DATABASE_PORT")
            .expect("DATABASE_PORT must be set")
            .parse()
            .expect("DATABASE_PORT must be a number");
        let database_user = var("DATABASE_USER").expect("DATABASE_USER must be set");
        let database_password = var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
        let database_name = var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        CoreConfiguration {
            app_redircet_send_verify_email_url,
            keycloak_admin_username: Arc::new(keycloak_admin_username),
            keycloak_admin_password: Arc::new(keycloak_admin_password),
            keycloak_url,
            keycloak_realm,
            keycloak_client_id,
            keycloak_client_secret,
            database_host,
            database_port,
            database_user,
            database_password,
            database_name,
        }
    }

    pub fn from_env_arc() -> Arc<CoreConfiguration> {
        Arc::new(CoreConfiguration::from_env())
    }
}
