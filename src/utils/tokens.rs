use std::sync::Arc;
use anyhow::Context;
use serde::{Serialize, Deserialize};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}};
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use sqlx::postgres::PgPool;
use uuid::Uuid;
use time::{Duration, OffsetDateTime};
use crate::{
    AppState,
    utils::{
        Error,
        ReqResult,
        DeviceInfo
    },
    models::session_model::NewSession, services::auth_service::{add_user_session, remove_user_session, find_session}
};
use async_trait::async_trait;
use axum::{
    Extension,
    extract::FromRequestParts,
    http::{
        HeaderValue,
        request::Parts,
        header::AUTHORIZATION
    }
};

const PREFIX: &str = "Bearer ";
const ACCESS_LIFE_TIME: Duration = Duration::minutes(10);
const REFRESH_LIFE_TIME: Duration = Duration::days(30);

#[derive(Serialize)]
pub struct TokenPair {
    pub access: String,
    pub refresh: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub jti: Uuid,
    pub aud: String,
    pub user_id: Uuid,
    pub exp: usize,
    pub iat: usize
}

pub struct AuthUser {
    pub user_id: Uuid,
    pub session_id: Uuid
}

pub struct MaybeAuthUser(pub Option<AuthUser>);

impl TokenPair {
    pub async fn new(
        pool: &PgPool,
        key: &RsaPrivateKey,
        session: NewSession
    ) -> ReqResult<TokenPair> {
        let key = key.to_pkcs8_pem(LineEnding::default())
            .context("Error parsing key")?;
        let key = EncodingKey::from_rsa_pem(key.as_bytes())
            .context("Error parsing key")?;

        let header = Header::new(Algorithm::RS256);
        let jti = Uuid::new_v4();
        let session = session.to_base(jti);

        let now = OffsetDateTime::now_utc();
        let iat = now.unix_timestamp() as usize;
        let access_exp = (now + ACCESS_LIFE_TIME).unix_timestamp() as usize;
        let refresh_exp = (now + REFRESH_LIFE_TIME).unix_timestamp() as usize;

        let access_claims = Claims {
            jti,
            aud: "api".to_string(),
            user_id: session.user_id,
            exp: access_exp,
            iat
        };

        let refresh_claims = Claims {
            jti,
            aud: "refresh".to_string(),
            user_id: session.user_id,
            exp: refresh_exp,
            iat
        };

        let access_token = encode(&header, &access_claims, &key)
            .context("Error encoding access token")?;
        let refresh_token = encode(&header, &refresh_claims, &key)
            .context("Error encoding refresh token")?;

        log::warn!("CREATING NEW USER SESSION\nid: {}\nuser_id: {}", jti, session.user_id);
        add_user_session(pool, session).await?;

        Ok(TokenPair {
            access: access_token,
            refresh: refresh_token
        })
    }

    pub async fn delete(
        pool: &PgPool,
        user_id: Uuid,
        session_id: Uuid
    ) -> ReqResult<()> {
        log::warn!("DELETING USER SESSION\nid: {}", session_id);
        remove_user_session(pool, user_id, session_id).await?;
        Ok(())
    }

    pub async fn refresh(
        pool: &PgPool,
        priv_key: &RsaPrivateKey,
        pub_key: &RsaPublicKey,
        refresh_token: &str,
        info: DeviceInfo
    ) -> ReqResult<TokenPair> {
        let claims = Claims::parse(pub_key, refresh_token)?;
        if !find_session(pool, claims.jti).await? {
            return Err(Error::Unauthorized);
        }
        TokenPair::delete(pool, claims.user_id, claims.jti).await?;
        let session = info.to_session(claims.user_id);
        Ok(TokenPair::new(pool, priv_key, session).await?)
    }
}

impl Claims {
    fn parse(
        key: &RsaPublicKey,
        token: &str
    ) -> ReqResult<Self> {
        let key = key.to_public_key_pem(LineEnding::default())
            .context("Error parsing key")?;
        let key = DecodingKey::from_rsa_pem(key.as_bytes())
            .context("Error parsing key")?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["api", "refresh"]);
        Ok(
            decode(token, &key, &validation)
                .context("Error parsing token")?
                .claims
        )
    }
}

impl AuthUser {
    async fn from_authorization(
        state: &AppState,
        auth_header: &HeaderValue
    ) -> ReqResult<Self> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::error!("Can not convert header to string");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(PREFIX) {
            log::error!("Header does not start with `Bearer `");
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[PREFIX.len()..];

        let claims = Claims::parse(&state.keys.public, token)
            .map_err(|_| {
                Error::Unauthorized
            })?;
        
        if (claims.exp as i64) < OffsetDateTime::now_utc().unix_timestamp() {
            log::info!("Token expired");
            return Err(Error::Unauthorized);
        }

        if !find_session(&state.pool, claims.jti).await? {
            return Err(Error::Unauthorized);
        }

        sqlx::query!(
            r#"
            UPDATE user_session
            SET last_active = $2
            WHERE session_id = $1
            "#,
            claims.jti, OffsetDateTime::now_utc()
        )
            .execute(&state.pool)
            .await?;

        Ok(Self {
            user_id: claims.user_id,
            session_id: claims.jti
        })
    }
}

impl MaybeAuthUser {
    #[allow(unused)]
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
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let state: Extension<Arc<AppState>> = Extension::from_request_parts(req, _s)
            .await
            .expect("BUG: AppState was not added as an extension");
        
        let auth_header = req
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        AuthUser::from_authorization(state.as_ref(), auth_header).await
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for MaybeAuthUser
where
    B: Send + Sync
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let state: Extension<Arc<AppState>> = Extension::from_request_parts(req, _s)
            .await
            .expect("BUG: AppState was not added as an extension");

        match req.headers.get(AUTHORIZATION) {
            Some(h) => Ok(MaybeAuthUser(AuthUser::from_authorization(state.as_ref(), h).await.ok())),
            None => Ok(MaybeAuthUser(None))
        }
    }
}