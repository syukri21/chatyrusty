mod handlers;
mod model;
mod page_handlres;
mod server;

#[tokio::main]
async fn main() {
    server::run().await;
}
