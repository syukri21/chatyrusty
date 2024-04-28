use std::net::SocketAddr;

use axum::{
    extract::{ws::Message, ConnectInfo, State, WebSocketUpgrade},
    response::IntoResponse,
};

use axum_extra::{headers, TypedHeader};
use crossbeam_channel::{unbounded, Receiver, Sender};

pub async fn email_verified_ch_test_handler(
    State(channel): State<EmailVerifiedChannel>,
) -> impl IntoResponse {
    let (tx, _) = channel.channel();
    let result = tx.send(EmailVerifiedMessage {
        user_id: "1".to_string(),
        message: "Hello".to_string(),
    });

    match result {
        Ok(_) => return "Sended".to_string(),
        Err(e) => return format!("Error: {}", e),
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(channel): State<EmailVerifiedChannel>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    let msg = format!("{} at {} connected.", user_agent, addr);
    tracing::info!(msg);
    ws.on_upgrade(move |socket| handle_socket(socket, channel))
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket, cn: EmailVerifiedChannel) {
    let _ = socket.send(Message::Text(format!("Welcome!"))).await;
    let (_, rx) = cn.channel();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv() {
            let _ = socket
                .send(Message::Text(format!("{}: {}", msg.user_id, msg.message)))
                .await;
        }
    });
}

#[derive(Clone)]
pub struct EmailVerifiedChannel((Sender<EmailVerifiedMessage>, Receiver<EmailVerifiedMessage>));

pub struct EmailVerifiedMessage {
    pub user_id: String,
    pub message: String,
}

pub trait EmailVerifiedChannelTrait {
    fn new() -> Self;
    fn channel(&self) -> (Sender<EmailVerifiedMessage>, Receiver<EmailVerifiedMessage>);
}

impl EmailVerifiedChannelTrait for EmailVerifiedChannel {
    fn new() -> Self {
        let (tx, rx) = unbounded::<EmailVerifiedMessage>();
        Self((tx, rx))
    }

    fn channel(&self) -> (Sender<EmailVerifiedMessage>, Receiver<EmailVerifiedMessage>) {
        self.0.clone()
    }
}
