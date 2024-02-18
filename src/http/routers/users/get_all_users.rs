use std::sync::Arc;
use axum::{Extension, Json};
use crate::http::{models::user::User, AppState, AuthUser, HttpResult};

pub async fn get_all_users(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser
) -> HttpResult<Json<Vec<User>>> {
    let users = User::get_all(&state.pool).await?;
    Ok(Json(users))
}