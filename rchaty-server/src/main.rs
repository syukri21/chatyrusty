mod handlers;
mod model;
mod server;

#[tokio::main]
async fn main() {
    server::run().await;
}
