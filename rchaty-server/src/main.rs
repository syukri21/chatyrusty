use axum::{
    routing::{get, post},
    Router,
};
use handlers::signup;
use rchaty_core::{
    kcloak::{KcloakConfig, KcloakImpl},
    AuthImpl,
};
use tokio::net::TcpListener;
use tracing::info;

mod handlers;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let kcloak_config = KcloakConfig::from_env();
    let kcloak = KcloakImpl::new(kcloak_config)
        .await
        .expect("Error initializing kcloak");
    let auth = AuthImpl::new(kcloak);

    let app = Router::new()
        .route("/signup", post(signup::<AuthImpl>))
        .route("/home", get(|| async { "This is your home" }))
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
