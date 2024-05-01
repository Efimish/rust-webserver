use std::net::SocketAddr;
use tokio::net::TcpListener;
mod http;

pub async fn run() {
    let port: u16 = std::env::var("PORT")
        .expect("Can not read PORT env variable")
        .parse()
        .expect("Can not parse PORT env variable");
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    log::info!("Starting server on http://{addr}");
    axum::serve(
        listener,
        http::router()
            .await
            .into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}