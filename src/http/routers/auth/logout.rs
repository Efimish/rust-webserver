use std::sync::Arc;
use axum::Extension;
use crate::http::{HttpResult, AppState, AuthUser, TokenPair};

pub async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) -> HttpResult<()> {
    TokenPair::delete(&state.pool, user.user_id, user.session_id).await?;
    Ok(())
}