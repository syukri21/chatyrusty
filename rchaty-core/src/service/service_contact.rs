use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;

use crate::{db::repository::DB, kcloak_client::KcloakClient, model::TokenIntrospect, BaseError};

#[async_trait]
pub trait Contact {
    async fn show_contact_list(&self, _token: &str) -> Result<Vec<ContactItem>, BaseError>;
}

#[derive(Debug, Clone)]
pub struct ContactItem {
    pub id: i32,
    pub user_id: String,
    pub friend_id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

pub struct ContactImpl {
    db: Arc<dyn DB + Send + Sync>,
    kcloak_client: Arc<dyn KcloakClient + Send + Sync>,
}

impl ContactImpl {
    pub fn new(
        db: Arc<dyn DB + Send + Sync>,
        kcloak_client: Arc<dyn KcloakClient + Send + Sync>,
    ) -> Self {
        ContactImpl { db, kcloak_client }
    }
}

#[async_trait]
impl Contact for ContactImpl {
    async fn show_contact_list(&self, token: &str) -> Result<Vec<ContactItem>, BaseError> {
        let token_introspect: TokenIntrospect = self.kcloak_client.introspect(token).await?;
        let user_id = token_introspect
            .sub
            .ok_or(BaseError::new(401, "invalid token"))?;
        let contacts = self.db.get_contacts_by_user_id(&user_id).await?;
        Ok(contacts)
    }
}
