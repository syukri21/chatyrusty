mod handlers;
mod model;
mod page_handler;
mod server;

#[tokio::main]
async fn main() {
    server::run().await;
}
