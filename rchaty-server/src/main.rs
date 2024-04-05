use axum::{routing::get, Router};
use rchaty_core::run;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new().route(
        "/",
        get(|| async {
            let test = run().await;
            info!("test: {}", test);
            "Hello, World!"
        }),
    );
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to 0.0.0.0:3000");

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
