use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    response::{IntoResponse, Response},
};

use rchaty_core::{
    chatchannel::{
        master::{ChannelDataImpl, MasterChannel},
        model::{Author, ContentType, MessageData, MessageStatus},
    },
    Auth, EmailVerifiedMessage,
};
use rchaty_web::htmx::VerifiedEmailSuccess;

pub async fn mock_chat_handler_sender<S>(
    Path(user_id): Path<String>,
    State(state): State<S>,
) -> Response<Body>
where
    S: MasterChannel,
{
    let tx = state.tx(user_id.as_str()).await;
    let tx = match tx {
        Some(tx) => tx,
        None => {
            let msg = format!("channel does not exist for user_id: {}", user_id);
            tracing::error!(msg);
            return "channel does not exist".into_response();
        }
    };

    let data = {
        let id = "26e03da6-38f0-4951-b00b-16ef0ab0cd8b".to_string();
        let conversation_id = "world".to_string();
        let author = Author::new(
            "9925ce5d-6174-4fd7-b978-018976280eb1".to_owned(),
            user_id.to_owned(),
            "email_test@example.com".to_owned(),
            "https://gravatar.com/avatar/9925ce5d61744fd7b978018976280eb1".to_owned(),
        );
        let content = "test".to_string();
        let content_type = ContentType::Text;
        let status = MessageStatus::Sent;
        let created_at = "2022-01-01 00:00:00".to_string();
        MessageData::new(
            id,
            conversation_id,
            author,
            content,
            content_type,
            created_at,
            status,
        )
    };

    let msg = Arc::new(ChannelDataImpl::new_chat_msg(data));
    let resp = tx.send(msg);
    match resp {
        Ok(_) => {
            let msg = format!("sended data for user_id: {}", user_id);
            tracing::info!(msg)
        }
        Err(e) => {
            let msg = format!("send data failed: {:}", e);
            tracing::error!(msg)
        }
    }
    "ok".into_response()
}

pub async fn mock_email_checker_handler<S>(
    Path(user_id): Path<String>,
    State(cn): State<S>,
) -> impl IntoResponse
where
    S: Auth + Send + Sync + 'static,
{
    let tx = cn.get_email_channel();
    tracing::info!("user: {user_id}, sended");
    let res = tx.send(EmailVerifiedMessage {
        user_id: user_id.to_string(),
        message: VerifiedEmailSuccess::htmx(),
    });
    match res {
        Ok(_) => {
            let msg = format!("email verified for user_id: {}", user_id);
            tracing::info!(msg)
        }
        Err(e) => {
            let msg = format!("email verification failed: {:}", e);
            tracing::error!(msg)
        }
    }
    "ok"
}
