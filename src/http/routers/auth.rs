use crate::{
    http::{
        extractors::{AuthUser, RequestInfo, ValidatedJson},
        HttpContext, HttpResult,
    },
    logic::auth,
    models::http_models::{AuthResponse, LoginBody, RefreshBody, RegisterBody},
    utils::tokens::TokenPair,
};
use axum::{routing::post, Extension, Json, Router};
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

pub async fn login(
    Extension(ctx): Extension<Arc<HttpContext>>,
    info: RequestInfo,
    ValidatedJson(body): ValidatedJson<LoginBody>,
) -> HttpResult<Json<AuthResponse>> {
    let response = auth::login(&ctx, body, info).await?;
    Ok(Json(response))
}

pub async fn register(
    Extension(ctx): Extension<Arc<HttpContext>>,
    info: RequestInfo,
    ValidatedJson(body): ValidatedJson<RegisterBody>,
) -> HttpResult<Json<AuthResponse>> {
    let response = auth::register(&ctx, body, info).await?;
    Ok(Json(response))
}

pub async fn refresh(
    Extension(ctx): Extension<Arc<HttpContext>>,
    info: RequestInfo,
    ValidatedJson(body): ValidatedJson<RefreshBody>,
) -> HttpResult<Json<TokenPair>> {
    let response = auth::refresh(&ctx, body, info).await?;
    Ok(Json(response))
}

pub async fn logout(Extension(ctx): Extension<Arc<HttpContext>>, user: AuthUser) -> HttpResult<()> {
    auth::logout(&ctx, user).await?;
    Ok(())
}
