use axum::{
    routing::{get, post},
    Router,
};
use handlers::{signin, signup};
use rchaty_core::{
    configuration::CoreConfiguration, kcloak::KcloakImpl, kcloak_client::KcloakClientImpl, AuthImpl,
};
use tokio::net::TcpListener;
use tracing::info;

use crate::handlers::{revoke_token, send_verify_email};

mod handlers;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Initialize CoreConfiguration
    let config = CoreConfiguration::from_env_arc();

    // Initialize Auth
    let auth = {
        // Initialize Kcloak Client
        let kcloak_client =
            KcloakClientImpl::new(config.clone().into()).expect("Error initializing kcloak client");

        // Initialize Kcloak Admin
        let kcloak = KcloakImpl::new(config.clone().into())
            .await
            .expect("Error initializing kcloak");

        AuthImpl::new(kcloak, kcloak_client)
    };

    let app = Router::new()
        .route("/signup", post(signup::<AuthImpl>))
        .route("/signin", post(signin::<AuthImpl>))
        .route("/send-verify-email", get(send_verify_email::<AuthImpl>))
        .route("/revoke-token", post(revoke_token::<AuthImpl>))
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
