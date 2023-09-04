mod core;
mod infra;

use infra::http;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    http::server::init().await
}
