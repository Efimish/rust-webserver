mod models;
mod http;
mod utils;
mod logic;

use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run() {
    let port: u16 = std::env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("failed to parse PORT env variable");
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    log::info!("Starting server on http://{addr}");
    axum::serve(
        listener,
        http::routers::main()
            .await
            .into_make_service_with_connect_info::<SocketAddr>()
    ).await.expect("failed to start the server");
}