use std::{net::SocketAddr, sync::Arc};
use axum::{Router, routing::post, http::HeaderMap, Json, extract::ConnectInfo, Extension};
use validator::Validate;
use crate::{AppState, utils::{AuthUser, MaybeAuthUser, TokenPair, RequestInfo, hash_password, verify_password, Error, ReqResult}};
use crate::services::user_service::add_user;
use crate::models::user_model::{
    LoginBody,
    RegisterBody,
    RefreshBody,
    AddUser
};

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

async fn login(
    Extension(state): Extension<Arc<AppState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    user: MaybeAuthUser,
    body: Json<LoginBody>
) -> ReqResult<Json<TokenPair>> {
    let user_ip = connect_info.to_string();
    let user_ip = user_ip.split_once(":").unwrap().0.to_string();
    let user_agent = headers["user-agent"].to_str().unwrap();
    let user_agent = RequestInfo::get(&user_ip, user_agent).await?;
    let user_location = format!("{}, {}", user_agent.country, user_agent.city);
    let user_agent = user_agent.os;

    let username = body.username.to_lowercase();
    let found_user = sqlx::query!(
        r#"SELECT * FROM "user" WHERE username = $1"#,
        username)
        .fetch_one(&state.pool)
        .await?;

    let user_id = found_user.user_id;
    let password_hash = found_user.password_hash;

    verify_password(body.password.clone(), password_hash).await?;

    log::debug!("----- Logging in -----\nUsername: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.password, user_ip, user_agent);

    if let Some(u) = user.user_id() {
        TokenPair::delete(&state.pool, u).await?;
    }
    let tokens = TokenPair::new(&state.pool, &state.keys.private,
        user_id, user_ip, user_agent, user_location).await?;

    Ok(Json(tokens))
}

async fn register(
    Extension(state): Extension<Arc<AppState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Json<RegisterBody>
) -> ReqResult<Json<TokenPair>> {
    body.validate()?;
    
    let user_ip = connect_info.to_string();
    let user_ip = user_ip.split_once(":").unwrap().0.to_string();
    let user_agent = headers["user-agent"].to_str().unwrap();
    let user_agent = RequestInfo::get(&user_ip, user_agent).await?;
    let user_location = format!("{}, {}", user_agent.country, user_agent.city);
    let user_agent = user_agent.os;

    let username = body.username.to_lowercase();
    let email = body.email.to_lowercase();
    let password_hash = hash_password(body.password.clone()).await?;
    log::debug!("----- Registering -----\nUsername: {}\nEmail: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.email, body.password, user_ip, user_agent);

    let user = AddUser {
        username,
        email,
        password_hash
    };

    let user_id = add_user(&state.pool, user).await?;

    let tokens = TokenPair::new(&state.pool, &state.keys.private,
        user_id, user_ip, user_agent, user_location).await?;
    
    Ok(Json(tokens))
}

async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Json<RefreshBody>
) -> ReqResult<Json<TokenPair>> {
    let user_ip = connect_info.to_string();
    let user_ip = user_ip.split_once(":").unwrap().0.to_string();
    let user_agent = headers["user-agent"].to_str().unwrap();
    let user_agent = RequestInfo::get(&user_ip, user_agent).await?;
    let user_location = format!("{}, {}", user_agent.country, user_agent.city);
    let user_agent = user_agent.os;

    log::debug!("----- Refreshing tokens -----\nIp: {}\nOS: {}", user_ip, user_agent);
    let tokens = TokenPair::refresh(&state.pool, &state.keys.private, &state.keys.public,
        &body.refresh_token, user_ip, user_agent, user_location).await?;
    
    Ok(Json(tokens))
}

async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<()> {
    TokenPair::delete(&state.pool, user.session_id).await?;
    Ok(())
}