use std::sync::Arc;
use axum::{Extension, Json};
use crate::http::{models::user::MyUser, AppState, AuthUser, HttpResult};

pub async fn get_my_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser,
) -> HttpResult<Json<MyUser>> {
    log::debug!("my user id = {}", user.user_id);
    let user = MyUser::get(&state.pool, user.user_id).await?;
    Ok(Json(user))
}