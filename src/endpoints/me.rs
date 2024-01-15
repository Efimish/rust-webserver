use std::sync::Arc;
use axum::{Router, Extension, Json, routing::get};
use crate::{
    AppState,
    utils::{AuthUser, ReqResult},
    models::user_model::FullUser,
    services::user_service::get_full_user
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(get_my_user)
        )
}

async fn get_my_user(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> ReqResult<Json<FullUser>> {
    let user = get_full_user(&state.pool, user.user_id).await?;
    Ok(Json(user))
}