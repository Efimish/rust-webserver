use std::sync::Arc;
use axum::{Json, extract::Path, Router, routing::get, Extension};
use crate::AppState;
use crate::utils::AuthUser;
use crate::models::user_model::BaseUser;
use crate::utils::Error;
use crate::services::user_service::{
    get_users as service_get_users,
    get_user_by_username as service_get_user_by_username
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/:username", get(get_user_by_username))
}

async fn get_users(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser
) -> Result<Json<Vec<BaseUser>>, Error> {
    Ok(Json(service_get_users(&state.pool).await?))
}

async fn get_user_by_username(
    Extension(state): Extension<Arc<AppState>>,
    Path(username): Path<String>,
    _: AuthUser
) -> Result<Json<BaseUser>, Error> {
    Ok(Json(service_get_user_by_username(&state.pool, &username).await?))
}