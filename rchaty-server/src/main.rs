mod handlers;
mod htmx_handler;
mod middleware;
mod model;
mod page_handler;
mod server;
mod ws_handler;
mod ws_mock_handler;

#[tokio::main]
async fn main() {
    server::run().await;
}
