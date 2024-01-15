use std::sync::Arc;
use axum::{Router, routing::post, Json, Extension};
use validator::Validate;
use crate::{AppState, utils::{AuthUser, MaybeAuthUser, TokenPair, DeviceInfo, hash_password, verify_password, ReqResult}};
use crate::services::user_service::{
    add_user,
    get_user_password
};
use crate::models::user_model::{
    RegisterBody,
    FixedRegisterBody,
    LoginBody,
    RefreshBody
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
    info: DeviceInfo,
    user: MaybeAuthUser,
    body: Json<LoginBody>
) -> ReqResult<Json<TokenPair>> {
    let username = body.username.to_lowercase();
    let found_user = get_user_password(&state.pool, &username).await?;
    let user_id = found_user.user_id;
    let password_hash = found_user.password_hash;
    verify_password(body.password.clone(), password_hash).await?;
    log::debug!("----- Logging in -----\nUsername: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.password, info.ip, info.os);
    if let Some(u) = user.0 {
        TokenPair::delete(&state.pool, u.user_id, u.session_id).await?;
    }
    let session = info.to_session(user_id);
    let tokens = TokenPair::new(
        &state.pool,
        &state.keys.private,
        session
    ).await?;
    Ok(Json(tokens))
}

async fn register(
    Extension(state): Extension<Arc<AppState>>,
    info: DeviceInfo,
    body: Json<RegisterBody>
) -> ReqResult<Json<TokenPair>> {
    body.validate()?;
    log::debug!("----- Registering -----\nUsername: {}\nEmail: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.email, body.password, info.ip, info.os);
    let user = FixedRegisterBody {
        username: body.username.to_lowercase(),
        email: body.email.to_lowercase(),
        password_hash: hash_password(body.password.clone()).await?
    };
    let user_id = add_user(&state.pool, user).await?;
    let session = info.to_session(user_id);
    let tokens = TokenPair::new(
        &state.pool,
        &state.keys.private,
        session
    ).await?;
    Ok(Json(tokens))
}

async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    info: DeviceInfo,
    body: Json<RefreshBody>
) -> ReqResult<Json<TokenPair>> {
    log::debug!("----- Refreshing tokens -----\nIp: {}\nOS: {}", info.ip, info.os);
    let tokens = TokenPair::refresh(
        &state.pool,
        &state.keys.private,
        &state.keys.public,
        &body.refresh_token,
        info
    ).await?;
    Ok(Json(tokens))
}

async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<()> {
    TokenPair::delete(&state.pool, user.user_id, user.session_id).await?;
    Ok(())
}