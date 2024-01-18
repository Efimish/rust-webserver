use axum::{Router, routing::post};
mod login;
mod register;
mod refresh;
mod logout;
use login::login;
use register::register;
use refresh::refresh;
use logout::logout;

pub fn router() -> Router {
    Router::new()
        .route(
            "/login",
            post(login)
        )
        .route(
            "/register",
            post(register)
        )
        .route(
            "/refresh",
            post(refresh)
        )
        .route(
            "/logout",
            post(logout)
        )
}