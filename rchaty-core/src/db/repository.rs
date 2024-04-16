use std::sync::Arc;

use async_trait::async_trait;
use keycloak::types::UserRepresentation;
use uuid::Uuid;

use crate::{configuration::CoreConfiguration, BaseError};

#[derive(Clone, Debug)]
pub struct DBConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl From<Arc<CoreConfiguration>> for DBConfig {
    fn from(core_config: Arc<CoreConfiguration>) -> Self {
        DBConfig {
            host: core_config.database_host.to_owned(),
            port: core_config.database_port,
            user: core_config.database_user.to_owned(),
            password: core_config.database_password.to_owned(),
            database: core_config.database_name.to_owned(),
        }
    }
}

pub struct DBImpl {
    pub config: Arc<DBConfig>,
    client: tokio_postgres::Client,
}

impl DBImpl {
    pub async fn connect(config: DBConfig) -> Self {
        let (client, connection) = tokio_postgres::connect(
            &format!(
                "host={} port={} user={} password={} dbname={}",
                config.host, config.port, config.user, config.password, config.database
            ),
            tokio_postgres::NoTls,
        )
        .await
        .expect("failed to connect to database");

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        DBImpl {
            config: Arc::new(config),
            client,
        }
    }
}

#[async_trait]
pub trait DB {
    async fn save_user(&self, user: &UserRepresentation) -> Result<(), BaseError>;
}

#[async_trait]
impl DB for DBImpl {
    async fn save_user(&self, user: &UserRepresentation) -> Result<(), BaseError> {
        let user_id = {
            let user_id = user.id.to_owned().unwrap();
            Uuid::parse_str(&user_id)
        }?;

        let client = &self.client;
        let row_affected = client
            .execute(
                "INSERT INTO users (user_id, first_name, last_name, email) VALUES ($1, $2, $3, $4)",
                &[&user_id, &user.first_name, &user.last_name, &user.email],
            )
            .await?;

        if row_affected == 0 {
            return Err(BaseError {
                code: 400,
                messages: "user already exists".to_string(),
            });
        }
        Ok(())
    }
}
