use axum::{routing::post, Router};
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
    let kcloak = KcloakImpl::new(kcloak_config);

    let auth = AuthImpl::new(kcloak.await);

    let app = Router::new()
        .route("/signup", post(signup::<AuthImpl>))
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
