use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{ws::Message, ConnectInfo, Path, State, WebSocketUpgrade},
    response::IntoResponse,
};

use axum_extra::{headers, TypedHeader};
use futures::{sink::SinkExt, StreamExt};
use rchaty_core::{chatchannel::master::MasterChannel, Auth, EmailVerifiedChannel};
use rchaty_web::htmx::ChatIncomming;

#[derive(Clone)]
pub struct WsAppState {
    pub email_verified_channel: Arc<dyn EmailVerifiedChannel + Send + Sync>,
}

pub async fn contact_list_handler<S>() -> impl IntoResponse {
    todo!("implement contact list ws handler")
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
