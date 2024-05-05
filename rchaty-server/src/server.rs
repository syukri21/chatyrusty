use std::{net::SocketAddr, sync::Arc};

use crate::{
    handlers::{callback_verify_email, revoke_token, send_verify_email, signin, signup},
    page_handler::{error_page, home_page, htmx_login_cliked, login_page, page_404, signup_page},
    ws_handler::{
        chat_handler, email_checker_handler, mock_chat_handler_sender, mock_email_checker_handler,
    },
};
use axum::{
    routing::{get, post},
    Router,
};
use rchaty_core::{
    chatchannel::master::MasterChannelImpl, configuration::CoreConfiguration,
    db::repository::DBImpl, kcloak::KcloakImpl, kcloak_client::KcloakClientImpl, AuthImpl,
    EmailVerifiedChannelImpl,
};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

pub async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Initialize CoreConfiguration
    let config = CoreConfiguration::from_env_arc();

    // Initialize DB
    let db = DBImpl::connect(Arc::clone(&config).into()).await;

    // Initialize EmailVerifiedChannel

    // Initialize Auth
    let auth = {
        // Initialize Kcloak Client
        let kcloak_client = KcloakClientImpl::new(Arc::clone(&config).into())
            .expect("Error initializing kcloak client");

        // Initialize Kcloak Adm n
        let kcloak = KcloakImpl::new(Arc::clone(&config).into())
            .await
            .expect("Error initializing kcloak");

        let email_channel = EmailVerifiedChannelImpl::new();

        AuthImpl::new(kcloak, kcloak_client, db, email_channel)
    };

    let master_channel = MasterChannelImpl::new();

    // Initialize Router api
    let api = Router::new()
        .route("/send-verify-email", get(send_verify_email::<AuthImpl>))
        .route("/revoke-token", post(revoke_token::<AuthImpl>));

    // Initialize Router htmx
    let htmx = Router::new().route("/login_clicked", get(htmx_login_cliked));
    let ws = Router::new()
        .route("/chat/:user_id", get(chat_handler::<MasterChannelImpl>))
        .route(
            "/mock/chat/:user_id",
            get(mock_chat_handler_sender::<MasterChannelImpl>),
        )
        .with_state(master_channel)
        .route("/vsc/:user_id", get(email_checker_handler::<AuthImpl>))
        .route(
            "/vsc_mock/:user_id",
            get(mock_email_checker_handler::<AuthImpl>),
        );

    let app = Router::new()
        .route("/error", get(error_page))
        .route("/login", get(login_page).post(signin::<AuthImpl>))
        .route("/signup", get(signup_page).post(signup::<AuthImpl>))
        .route("/home", get(home_page))
        .route("/", get(home_page))
        .route(
            "/callback-verified-email",
            get(callback_verify_email::<AuthImpl>),
        )
        .nest("/api/v1", api)
        .nest("/htmx", htmx)
        .nest("/ws", ws)
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(auth);

    let app = app.fallback(page_404);

    let host = "0.0.0.0";
    let port = 3000;
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .expect("Failed to bind to 0.0.0.0:3000");

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.layer(TraceLayer::new_for_http())
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to start server");
}
