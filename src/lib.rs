mod utils;
mod endpoints;
mod services;
mod models;
use std::{path::PathBuf, net::SocketAddr, sync::Arc};
use utils::RsaKeyPair;
use dotenv::dotenv;
use axum::{Router, Extension};
use tower_http::cors::{CorsLayer, Any};
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    keys: RsaKeyPair
}

async fn run() {
    dotenv().ok();
    let current_dir: PathBuf = std::env::current_dir()
        .expect("Can not access current directory");
    let data_dir: PathBuf = current_dir.join("data");
    match data_dir.try_exists() {
        Err(_) => panic!("error checking if data directory exists"),
        Ok(v) if !v => panic!("data directory does not exist"),
        Ok(_) => ()
    };
    let keys_dir: PathBuf = data_dir.join("keys");
    let keys = RsaKeyPair::read_or_generate(&keys_dir).unwrap();
    
    let db_url: String = std::env::var("DATABASE_URL")
        .expect("Can not read DATABASE_URL env variable");

    let pool: sqlx::Pool<sqlx::Postgres> = PgPool::connect(&db_url).await
        .expect("Can not connect to the database");

    let state = Arc::new(AppState {
        pool,
        keys
    });

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any);

    let app: Router = Router::new()
        .nest("/auth", endpoints::auth_router())
        .nest("/me", endpoints::me_router())
        .nest("/users", endpoints::users_router())
        .nest("/test", endpoints::test_router())
        .layer(cors)
        .layer(Extension(state));

    start(app).await;
}

async fn start(app: Router) {
    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Starting server on http://{}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}