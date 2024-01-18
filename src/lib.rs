use std::net::SocketAddr;
use tokio::net::TcpListener;
mod http;

pub async fn run() {
    let port = 3000;
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    log::info!("Starting server on http://{addr}");
    axum::serve(
        listener,
        http::router()
            .await
            .into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}