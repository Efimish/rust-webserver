use std::sync::Arc;
use axum::{Extension, Json, extract::Path};
use uuid::Uuid;
use crate::http::{models::user::User, AppState, AuthUser, HttpResult};

pub async fn get_user(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser,
    Path(user_id): Path<Uuid>
) -> HttpResult<Json<User>> {
    let user = User::get(&state.pool, user_id).await?;
    Ok(Json(user))
}