use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
}

impl Serialize for MessageStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl MessageStatus {
    pub fn to_string(&self) -> String {
        match self {
            MessageStatus::Sent => "sent".to_string(),
            MessageStatus::Delivered => "delivered".to_string(),
            MessageStatus::Read => "read".to_string(),
        }
    }

    pub fn from_string(status: &str) -> Self {
        match status {
            "sent" => MessageStatus::Sent,
            "delivered" => MessageStatus::Delivered,
            "read" => MessageStatus::Read,
            _ => MessageStatus::Sent,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageData {
    pub id: String,
    pub conversation_id: String,
    pub author: Author,
    pub content: String,
    pub content_type: ContentType,
    pub created_at: String,
    pub status: MessageStatus,
}

impl MessageData {
    pub fn new(
        id: String,
        conversation_id: String,
        author: Author,
        content: String,
        content_type: ContentType,
        created_at: String,
        status: MessageStatus,
    ) -> Self {
        Self {
            id,
            conversation_id,
            author,
            content,
            content_type,
            created_at,
            status,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Author {
    id: String,
    username: String,
    email: String,
    avatar: String,
}

impl Author {
    pub fn new(id: String, username: String, email: String, avatar: String) -> Self {
        Self {
            id,
            username,
            email,
            avatar,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ContentType {
    Text,
    Image,
}

impl Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ContentType {
    pub fn to_string(&self) -> String {
        match self {
            ContentType::Text => "text".to_string(),
            ContentType::Image => "image".to_string(),
        }
    }

    pub fn from_string(content_type: &str) -> Self {
        match content_type {
            "text" => ContentType::Text,
            "image" => ContentType::Image,
            _ => ContentType::Text,
        }
    }
}
