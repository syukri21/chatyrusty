mod handlers;
mod htmx_handler;
mod model;
mod page_handler;
mod server;

#[tokio::main]
async fn main() {
    server::run().await;
}
