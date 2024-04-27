use std::net::SocketAddr;

use axum::{
    extract::{ws::Message, ConnectInfo, WebSocketUpgrade},
    response::IntoResponse,
};

use axum_extra::{headers, TypedHeader};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    let msg = format!("{} at {} connected.", user_agent, addr);
    tracing::info!(msg);
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket) {
    let _ = socket
        .send(Message::Text(String::from("Welcome to the chat!")))
        .await;
}
