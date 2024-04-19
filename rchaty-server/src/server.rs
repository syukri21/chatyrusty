use std::sync::Arc;

use crate::{
    handlers::{callback_verify_email, revoke_token, send_verify_email, signin, signup},
    page_handlres::{error_page, login_page},
};
use axum::{
    routing::{get, post},
    Router,
};
use rchaty_core::{
    configuration::CoreConfiguration, db::repository::DBImpl, kcloak::KcloakImpl,
    kcloak_client::KcloakClientImpl, AuthImpl,
};
use tokio::net::TcpListener;
use tracing::info;

pub async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Initialize CoreConfiguration
    let config = CoreConfiguration::from_env_arc();

    // Initialize DB
    let db = DBImpl::connect(Arc::clone(&config).into()).await;

    // Initialize Auth
    let auth = {
        // Initialize Kcloak Client
        let kcloak_client = KcloakClientImpl::new(Arc::clone(&config).into())
            .expect("Error initializing kcloak client");

        // Initialize Kcloak Admin
        let kcloak = KcloakImpl::new(Arc::clone(&config).into())
            .await
            .expect("Error initializing kcloak");

        AuthImpl::new(kcloak, kcloak_client, db)
    };

    // Initialize Router api
    let api = Router::new()
        .route("/signup", post(signup::<AuthImpl>))
        .route("/signin", post(signin::<AuthImpl>))
        .route("/send-verify-email", get(send_verify_email::<AuthImpl>))
        .route("/revoke-token", post(revoke_token::<AuthImpl>))
        .route(
            "/callback-verified-email",
            get(callback_verify_email::<AuthImpl>),
        )
        .route("/home", get(|| async { "This is your home" }))
        .with_state(auth);
    let app = Router::new()
        .route("/error", get(error_page))
        .route("/login", get(login_page))
        .nest("/api/v1", api);

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
