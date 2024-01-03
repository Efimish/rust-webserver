use std::sync::Arc;
use serde::{Serialize, Deserialize};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding}};
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use sqlx::postgres::PgPool;
use uuid::Uuid;
use time::{Duration, OffsetDateTime};
use super::Error;
use crate::AppState;

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
// const ACCESS_LIFE_TIME: Duration = Duration::minutes(10);
const ACCESS_LIFE_TIME: Duration = Duration::seconds(20);
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
        pool: &PgPool, key: &RsaPrivateKey,
        user_id: Uuid, user_ip: String,
        user_agent: String, user_location: String
    ) -> TokenPair {
        let key = key.to_pkcs8_pem(LineEnding::default()).unwrap();
        let key = EncodingKey::from_rsa_pem(key.as_bytes()).unwrap();

        let header = Header::new(Algorithm::RS256);
        let jti = Uuid::new_v4();

        let now = OffsetDateTime::now_utc();
        let iat = now.unix_timestamp() as usize;
        let access_exp = (now + ACCESS_LIFE_TIME).unix_timestamp() as usize;
        let refresh_exp = (now + REFRESH_LIFE_TIME).unix_timestamp() as usize;

        let access_claims = Claims {
            jti,
            aud: "api".to_string(),
            user_id,
            exp: access_exp,
            iat
        };

        let refresh_claims = Claims {
            jti,
            aud: "refresh".to_string(),
            user_id,
            exp: refresh_exp,
            iat
        };

        let access_token = encode(&header, &access_claims, &key).unwrap();
        let refresh_token = encode(&header, &refresh_claims, &key).unwrap();

        sqlx::query!(
            r#"INSERT INTO user_session (
                user_id, session_id,
                user_ip, user_agent, user_location
            ) VALUES (
                $1, $2, $3, $4, $5
            )"#,
            user_id, jti,
            user_ip, user_agent, user_location
        ).execute(pool).await.unwrap();

        TokenPair {
            access: access_token,
            refresh: refresh_token
        }
    }

    pub async fn delete(pool: &PgPool, session_id: Uuid) {
        let res = sqlx::query!(
            r#"DELETE FROM user_session WHERE session_id = $1"#,
            session_id
        ).execute(pool).await.unwrap();
        println!("Old session deleted, {} rows affected", res.rows_affected());
    }

    pub async fn refresh(
        pool: &PgPool, priv_key: &RsaPrivateKey, pub_key: &RsaPublicKey,
        refresh_token: &str, user_ip: String, user_agent: String, user_location: String
    ) -> Result<TokenPair, Error> {
        let claims = Claims::parse(pub_key, refresh_token).unwrap();
        let token_exists = sqlx::query!(
            r#"SELECT COUNT(1) FROM user_session WHERE session_id = $1"#,
            claims.jti
        ).fetch_one(pool).await.unwrap().count.unwrap() == 1;
        
        if !token_exists {
            return Err(Error::Unauthorized);
        }
        TokenPair::delete(pool, claims.jti).await;
        Ok(TokenPair::new(pool, priv_key, claims.user_id, user_ip, user_agent, user_location).await)
    }
}

impl Claims {
    fn parse(key: &RsaPublicKey, token: &str) -> Result<Self, &'static str> {
        let key = key.to_public_key_pem(LineEnding::default()).unwrap();
        let key = DecodingKey::from_rsa_pem(key.as_bytes()).unwrap();
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["api", "refresh"]);
        decode(
            token, &key, &validation
        ).map_err(|_| {
            println!("Can not parse token");
            "Can not parse token"
        }).map(|t| {
            t.claims
        })
    }
}

impl AuthUser {
    async fn from_authorization(state: &AppState, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            println!("Can not convert header to string");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(PREFIX) {
            println!("Header does not start with `Bearer `");
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[PREFIX.len()..];

        let claims = Claims::parse(&state.keys.public, token)
            .map_err(|_| {
                Error::Unauthorized
            })?;
        
        if (claims.exp as i64) < OffsetDateTime::now_utc().unix_timestamp() {
            println!("Token expired");
            return Err(Error::Unauthorized);
        }

        if sqlx::query!(
            r#"SELECT COUNT(1) FROM user_session WHERE session_id = $1"#, claims.jti
        ).fetch_one(&state.pool).await.unwrap().count.unwrap() == 0 {
            return Err(Error::Unauthorized);
        }

        sqlx::query!(
            r#"UPDATE user_session
            SET last_active = $2
            WHERE session_id = $1"#,
            claims.jti, OffsetDateTime::now_utc()
        ).execute(&state.pool).await.unwrap();

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