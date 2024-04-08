use axum::{routing::get, Router};
use handlers::signup;
use rchaty_core::AuthImpl;
use tokio::net::TcpListener;
use tracing::info;

mod handlers;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let auth = AuthImpl::new();

    let app = Router::new()
        .route("/signup", get(signup::<AuthImpl>))
        .with_state(auth);

    let host = "0.0.0.0";
    let port = 3000;
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .expect("Failed to bind to 0.0.0.0:3000");

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
