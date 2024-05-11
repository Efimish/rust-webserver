//! Extractors of user authentication information.

use std::sync::Arc;
use axum::{
    Extension,
    extract::FromRequestParts,
    http::{
        request::Parts,
        HeaderValue,
        header::AUTHORIZATION
    }
};
use time::OffsetDateTime;
use sqlx::PgPool;
use uuid::Uuid;
use async_trait::async_trait;
use crate::{
    http::{
        HttpResult,
        HttpError,
        HttpContext
    },
    utils::tokens::Claims
};

/// # User must be authenticated
pub struct AuthUser {
    pub user_id: Uuid,
    pub session_id: Uuid
}

/// # User could be authenticated
pub struct MaybeAuthUser(pub Option<AuthUser>);

const PREFIX: &str = "Bearer ";

impl AuthUser {
    async fn from_authorization(
        pool: &PgPool,
        auth_header: &HeaderValue
    ) -> HttpResult<Self> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::error!("failed to convert auth header to string");
            HttpError::Unauthorized
        })?;

        if !auth_header.starts_with(PREFIX) {
            log::error!("Header does not start with `{PREFIX}`");
            return Err(HttpError::Unauthorized);
        }

        let token = &auth_header[PREFIX.len()..];

        let claims = Claims::parse(token)
            .map_err(|_| {
                HttpError::Unauthorized
            })?;
        
        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::info!("Token expired");
            return Err(HttpError::Unauthorized);
        }

        if sqlx::query!(
            r#"
            SELECT COUNT(1)
            FROM user_session
            WHERE id = $1
            AND user_id = $2
            "#,
            claims.jti,
            claims.user_id
        )
        .fetch_one(pool)
        .await?
        .count != Some(1) {
            return Err(HttpError::Unauthorized);
        }

        sqlx::query!(
            r#"
            UPDATE user_session
            SET last_active = $2
            WHERE id = $1
            "#,
            claims.jti, OffsetDateTime::now_utc()
        )
            .execute(pool)
            .await?;

        Ok(Self {
            user_id: claims.user_id,
            session_id: claims.jti
        })
    }
}

impl MaybeAuthUser {
    pub fn user_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|user| user.user_id)
    }
    pub fn session_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|user| user.session_id)
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for AuthUser
where
    B: Send + Sync
{
    type Rejection = HttpError;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let state: Extension<Arc<HttpContext>> = Extension::from_request_parts(req, _s)
            .await
            .expect("BUG: HttpContext was not added as an extension");
        
        let auth_header = req
            .headers
            .get(AUTHORIZATION)
            .ok_or(HttpError::Unauthorized)?;

        Self::from_authorization(&state.pool, auth_header).await
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for MaybeAuthUser
where
    B: Send + Sync
{
    type Rejection = HttpError;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let state: Extension<Arc<HttpContext>> = Extension::from_request_parts(req, _s)
            .await
            .expect("BUG: HttpContext was not added as an extension");

        Ok(Self(
            if let Some(header) = req.headers.get(AUTHORIZATION) {
                AuthUser::from_authorization(&state.pool, header).await.ok()
            } else {
                None
            }
        ))
    }
}