use std::sync::Arc;
use axum::{Json, Extension};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;
use crate::http::{HttpResult, HttpError, DeviceInfo, TokenPair, AppState, helpers::password::hash_password};

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^\w+$").unwrap();
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBody {
    #[validate(
        length(
            min = 3,
            max = 24,
            message = "Username must be between 3 and 24 characters"
        ),
        regex(
            path = "USERNAME_REGEX",
            message = "Username must only contain english letters, numbers and unserscore"
        )
    )]
    pub username: String,
    #[validate(
        email(
            message = "Email must be valid"
        )
    )]
    pub email: String,
    #[validate(
        length(
            min = 3,
            message = "Password must be at least 3 characters"
        )
    )]
    pub password: String
}

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    info: DeviceInfo,
    body: Json<RegisterBody>
) -> HttpResult<Json<TokenPair>> {
    body.validate()?;
    log::debug!("----- Registering -----\nUsername: {}\nEmail: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.email, body.password, info.ip, info.os);
    let username = body.username.to_lowercase();
    let email = body.email.to_lowercase();
    let password_hash = hash_password(body.password.clone()).await?;

    if sqlx::query!(
        r#"
        SELECT COUNT(1) FROM "user"
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(&state.pool)
    .await?
    .count == Some(1) {
        return Err(HttpError::BadRequest);
    }

    let user_id = sqlx::query!(
        r#"
        INSERT INTO "user" (
            username,
            email,
            password_hash,
            display_name
        ) VALUES (
            $1, $2, $3, $1
        ) RETURNING id
        "#,
        username,
        email,
        password_hash
    )
        .fetch_one(&state.pool)
        .await?
        .id;
    
    let tokens = TokenPair::new(
        &state.pool,
        info,
        user_id
    ).await?;
    
    Ok(Json(tokens))
}