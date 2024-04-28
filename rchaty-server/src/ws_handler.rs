use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{ws::Message, ConnectInfo, Path, State, WebSocketUpgrade},
    response::IntoResponse,
};

use axum_extra::{headers, TypedHeader};

use crate::channel::{EmailVerifiedChannel, EmailVerifiedMessage};

#[derive(Clone)]
pub struct WsAppState {
    pub email_verified_channel: Arc<dyn EmailVerifiedChannel + Send + Sync>,
}

pub async fn mock_email_checker_handler<S>(
    Path(user_id): Path<String>,
    State(cn): State<S>,
) -> impl IntoResponse
where
    S: EmailVerifiedChannel + Send + Sync + 'static,
{
    let tx = cn.sender();
    tracing::info!("user: {user_id}, sended");
    let res = tx.send(EmailVerifiedMessage {
        user_id: user_id.clone(),
        message: "verified".to_string(),
    });
    match res {
        Ok(_) => return format!("user: {user_id}, sended"),
        Err(e) => {
            tracing::error!("Error sending message: {}", e);
            return format!("user: {user_id} Error sending message: {e}");
        }
    }
}

pub async fn email_checker_handler<S>(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(user_id): Path<String>,
    State(channel): State<S>,
) -> impl IntoResponse
where
    S: EmailVerifiedChannel + Send + Sync + 'static,
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

async fn email_checker_handler_socket<T: EmailVerifiedChannel + Send + Sync>(
    mut socket: axum::extract::ws::WebSocket,
    user_id: String,
    cn: T,
) {
    let mut rx = cn.receiver();
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
