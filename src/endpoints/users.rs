use std::sync::Arc;
use axum::{Json, extract::Path, Router, routing::get, Extension};
use crate::AppState;
use crate::utils::AuthUser;
use crate::models::user_model::BaseUser;
use crate::utils::ReqResult;
use crate::services::user_service::{
    get_users as service_get_users,
    get_user as service_get_user
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(get_users)
        )
        .route(
            "/:username",
            get(get_user)
        )
}

async fn get_users(
    Extension(state): Extension<Arc<AppState>>,
    _: AuthUser
) -> ReqResult<Json<Vec<BaseUser>>> {
    Ok(Json(service_get_users(&state.pool).await?))
}

async fn get_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(username): Path<String>,
    _: AuthUser
) -> ReqResult<Json<BaseUser>> {
    Ok(Json(service_get_user(&state.pool, &username).await?))
}