use std::sync::Arc;
use axum::{Json, Extension};
use serde::Deserialize;
use utoipa::ToSchema;
use crate::http::routers::AppState;
use crate::http::HttpResult;
use crate::http::{TokenPair, DeviceInfo};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshBody {
    refresh_token: String
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    request_body = RefreshBody,
    responses(
        (status = OK, description = "Refreshes your session", body = TokenPair)
    ),
    tag = "users"
)]
pub async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    info: DeviceInfo,
    body: Json<RefreshBody>
) -> HttpResult<Json<TokenPair>> {
    log::debug!("----- Refreshing tokens -----\nIp: {}\nOS: {}", info.ip, info.os);
    let tokens = TokenPair::refresh(
        &state.pool,
        &body.refresh_token,
        info
    ).await?;
    Ok(Json(tokens))
}