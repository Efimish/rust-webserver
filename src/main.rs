//! Init necessary things and start the server

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    webserver::run().await
}