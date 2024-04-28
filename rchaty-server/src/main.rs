mod channel;
mod handlers;
mod htmx_handler;
mod model;
mod page_handler;
mod server;
mod ws_handler;

#[tokio::main]
async fn main() {
    server::run().await;
}
