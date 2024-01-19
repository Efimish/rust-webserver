use std::sync::Arc;
use axum::Extension;
use crate::http::routers::AppState;
use crate::http::{HttpResult, AuthUser};
use crate::http::TokenPair;

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = OK, description = "Logs you out and deletes session")
    ),
    tag = "users"
)]
pub async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> HttpResult<()> {
    TokenPair::delete(&state.pool, user.user_id, user.session_id).await?;
    Ok(())
}