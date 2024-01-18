use std::sync::Arc;
use axum::{Json, Extension};
use serde::Deserialize;
use crate::http::{AppState, HttpResult, HttpError, DeviceInfo, MaybeAuthUser, TokenPair, password::verify_password};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginBody {
    pub username: String,
    pub password: String
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    info: DeviceInfo,
    user: MaybeAuthUser,
    body: Json<LoginBody>
) -> HttpResult<Json<TokenPair>> {
    let username = body.username.to_lowercase();
    if sqlx::query!(
        r#"
        SELECT COUNT(1) FROM "user"
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(&state.pool)
    .await?
    .count != Some(1) {
        return Err(HttpError::BadRequest);
    }
    let found_user = sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM "user"
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(&state.pool)
    .await?;

    let user_id = found_user.user_id;
    let password_hash = found_user.password_hash;

    verify_password(body.password.clone(), password_hash).await?;

    log::debug!("----- Logging in -----\nUsername: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.password, info.ip, info.os);

    if let Some(u) = user.0 {
        TokenPair::delete(&state.pool, u.user_id, u.session_id).await?;
    }

    let tokens = TokenPair::new(
        &state.pool,
        info,
        user_id
    ).await?;

    Ok(Json(tokens))
}