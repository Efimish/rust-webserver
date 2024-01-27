use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;

use crate::http::{HttpResult, HttpError, error::ResultExt, AppState, AuthUser, password::hash_password};
use super::get_my_user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditUserBody {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub display_name: Option<String>,
    pub status: Option<String>
}

pub async fn edit_my_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
    body: Json<EditUserBody>
) -> HttpResult<Json<User>> {
    let password_hash = if let Some(password) = body.password.clone() {
        Some(hash_password(password).await?)
    } else {
        None
    };
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE "user"
        SET username = coalesce($1, "user".username),
            email = coalesce($2, "user".email),
            password_hash = coalesce($3, "user".password_hash),
            display_name = coalesce($4, "user".display_name),
            status = coalesce($5, "user".status)
        WHERE user_id = $6
        RETURNING user_id, username, email, display_name, avatar, status
        "#,
        body.username,
        body.email,
        password_hash,
        body.display_name,
        body.status,
        user.user_id
    )
    .fetch_one(&state.pool)
    .await
    .on_constraint("user_username_key", |_| {
        HttpError::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        HttpError::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(user))
}