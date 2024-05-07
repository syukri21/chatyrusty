use std::{net::SocketAddr, sync::Arc};

use crate::{
    handlers::{callback_verify_email, revoke_token, send_verify_email, signin, signup},
    htmx_handler::check_auth,
    page_handler::{error_page, home_page, htmx_login_cliked, login_page, page_404, signup_page},
    ws_handler::{
        chat_handler, email_checker_handler, mock_chat_handler_sender, mock_email_checker_handler,
    },
};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use rchaty_core::{
    chatchannel::master::MasterChannelImpl,
    configuration::CoreConfiguration,
    db::repository::DBImpl,
    kcloak::KcloakImpl,
    kcloak_client::{KcloakClient, KcloakClientImpl},
    model::TokenIntrospect,
    AuthImpl, EmailVerifiedChannelImpl,
};
use rchaty_web::htmx::RedirectHtmx;
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

    // Initialize Kcloak Client
    let kcloak_client = KcloakClientImpl::new(Arc::clone(&config).into())
        .expect("Error initializing kcloak client");
    let arc_kcloak_client = Arc::new(kcloak_client);

    // Initialize Auth
    let auth = {
        // Initialize Kcloak Adm n
        let kcloak = KcloakImpl::new(Arc::clone(&config).into())
            .await
            .expect("Error initializing kcloak");

        let email_channel = EmailVerifiedChannelImpl::new();

        AuthImpl::new(kcloak, Arc::clone(&arc_kcloak_client), db, email_channel)
    };

    let master_channel = MasterChannelImpl::new();

    let guard_htmx_auth = Box::new(middleware::from_fn_with_state(
        Arc::clone(&arc_kcloak_client),
        auth_htmx_middleware,
    ));

    // Initialize Router api
    let api = Router::new()
        .route("/send-verify-email", get(send_verify_email::<AuthImpl>))
        .route("/revoke-token", post(revoke_token::<AuthImpl>));

    // Initialize Router htmx
    let htmx = Router::new()
        .route("/login_clicked", get(htmx_login_cliked))
        .route(
            "/check_auth",
            get(check_auth).layer(*guard_htmx_auth.clone()),
        );

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

    let app = app
        .layer(TraceLayer::new_for_http())
        .into_make_service_with_connect_info::<SocketAddr>();

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn auth_htmx_middleware(
    headers: HeaderMap,
    State(state): State<Arc<KcloakClientImpl>>,
    request: Request,
    next: Next,
) -> Response<Body> {
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return RedirectHtmx::htmx("/login").into_response(),
    };

    let token = token.replace("Bearer ", "");
    tracing::info!("token: {:?}", token);
    let introspect = state
        .introspect(&token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED);

    let introspect = match introspect {
        Ok(introspect) => introspect,
        Err(_) => return RedirectHtmx::htmx("/login").into_response(),
    };

    tracing::info!("introspect: {:?}", introspect.name);
    if !introspect.active {
        return RedirectHtmx::htmx("/login").into_response();
    }

    next.run(request).await
}

#[warn(dead_code)]
async fn auth_middleware(
    headers: HeaderMap,
    State(state): State<Arc<KcloakClientImpl>>,
    request: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return Err(StatusCode::UNAUTHORIZED),
    }
    .replace("Bearer ", "");

    let introspect = state
        .introspect(&token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    tracing::info!("introspect: {:?}", introspect);
    if !introspect.active {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
