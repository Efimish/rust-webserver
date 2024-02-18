use std::sync::Arc;
use axum::{Extension, extract::Path, Json};
use crate::http::{models::user::User, AppState, AuthUser, HttpResult};

pub async fn get_user_by_username(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
    Path(username): Path<String>
) -> HttpResult<Json<User>> {
    let user = User::get_by_username(&state.pool, username).await?;
    Ok(Json(user))
}