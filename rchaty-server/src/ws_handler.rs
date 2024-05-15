use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    extract::{ws::Message, ConnectInfo, Path, State, WebSocketUpgrade},
    response::{IntoResponse, Response},
};

use axum_extra::{headers, TypedHeader};
use futures::{sink::SinkExt, StreamExt};
use rchaty_core::{
    chatchannel::{
        master::{ChannelDataImpl, MasterChannel},
        model::{Author, ContentType, MessageData, MessageStatus},
    },
    Auth, EmailVerifiedChannel, EmailVerifiedMessage,
};
use rchaty_web::htmx::{ChatIncomming, VerifiedEmailSuccess};

#[derive(Clone)]
pub struct WsAppState {
    pub email_verified_channel: Arc<dyn EmailVerifiedChannel + Send + Sync>,
}

pub async fn email_checker_handler<S>(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(user_id): Path<String>,
    State(channel): State<S>,
) -> impl IntoResponse
where
    S: Auth + Send + Sync + 'static,
{
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    let msg = format!(
        "user: {}, agent:{} at {} connected.",
        user_id, user_agent, addr
    );
    tracing::info!(msg);
    ws.on_upgrade(move |socket| email_checker_handler_socket::<S>(socket, user_id, channel))
}

async fn email_checker_handler_socket<T: Auth + Send + Sync>(
    mut socket: axum::extract::ws::WebSocket,
    user_id: String,
    cn: T,
) {
    let mut rx = cn.get_email_channel().receiver();
    loop {
        let msg = rx.recv().await.unwrap();
        tracing::info!("{}: {}", user_id, msg.message);
        if user_id == msg.user_id {
            socket
                .send(Message::Text(format!("{}: {}", msg.user_id, msg.message)))
                .await
                .unwrap();
            break;
        }
    }
}

pub async fn chat_handler<S>(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(user_id): Path<String>,
    State(state): State<S>,
) -> impl IntoResponse
where
    S: MasterChannel,
{
    tracing::info!(
        "user: {}, agent: {:?} at {} connected.",
        user_id,
        user_agent,
        addr
    );

    state.create_channel(user_id.as_str());
    let tx = state.tx(user_id.as_str()).await;
    let tx = match tx {
        Some(tx) => tx,
        None => {
            let msg = format!("channel does not exist for user_id: {}", user_id);
            tracing::error!(msg);
            return "channel does not exist".into_response();
        }
    };
    ws.on_upgrade(|socket| async move {
        let mut rx = tx.subscribe();
        let (mut sender, mut _receiver) = socket.split();

        let _send_task = tokio::spawn(async move {
            loop {
                let msg = rx.recv().await;
                if !msg.is_ok() {
                    break;
                }
                let msg = msg.unwrap();
                let msg = ChatIncomming::htmx(&msg.content(), &msg.created_at());
                sender.send(Message::Text(msg)).await.unwrap();
            }
        });
    })
}

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

// Mock handler
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
