use std::{path::PathBuf, sync::Arc};
use async_trait::async_trait;
use axum::{http::{HeaderValue, request::Parts, header::AUTHORIZATION}, extract::FromRequestParts, Extension};
use serde::{Serialize, Deserialize};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey, LineEnding}};
use anyhow::Context;
use lazy_static::lazy_static;
use jsonwebtoken::{EncodingKey, DecodingKey, Validation, Algorithm, Header};
use time::{Duration, OffsetDateTime};
use sqlx::PgPool;
use uuid::Uuid;
use super::{HttpResult, HttpError, DeviceInfo, AppState};

pub struct AuthUser {
    pub user_id: Uuid,
    pub session_id: Uuid
}

pub struct MaybeAuthUser(pub Option<AuthUser>);

const PREFIX: &str = "Bearer ";
const ACCESS_LIFE_TIME: Duration = Duration::minutes(10);
const REFRESH_LIFE_TIME: Duration = Duration::days(30);

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub jti: Uuid,
    pub aud: String,
    pub user_id: Uuid,
    pub exp: usize,
    pub iat: usize
}

#[derive(Serialize)]
pub struct TokenPair {
    pub access: String,
    pub refresh: String
}

#[derive(Clone)]
struct RsaKeyPair {
    private: RsaPrivateKey,
    public: RsaPublicKey
}

impl RsaKeyPair {
    fn read_or_generate(dir_path: &str) -> anyhow::Result<Self> {
        let current_dir: PathBuf = std::env::current_dir()
            .expect("Can not access current directory");
        let keys_dir: PathBuf = current_dir.join(dir_path);
        anyhow::ensure!(
            keys_dir
            .try_exists()
            .context("Error checking if keys directory exists")?,
            "keys directory does not exist"
        );
        let private_key_file: PathBuf = keys_dir.join("private.key");
        let public_key_file: PathBuf = keys_dir.join("public.key");

        if !private_key_file
            .try_exists()
            .context("Error checking if private key file directory exists")?
        || !public_key_file
            .try_exists()
            .context("Error checking if public key file directory exists")? {
            
            log::info!("Keys not found, generating new key pair");
    
            let mut rng = rand::thread_rng();
            let bits = 2048;
            let private_key = RsaPrivateKey::new(&mut rng, bits)
                .context("Error creating private key")?;
            let public_key = RsaPublicKey::from(&private_key);
    
            private_key.write_pkcs8_pem_file(&private_key_file, LineEnding::default())
                .context("Error writing private key")?;
            public_key.write_public_key_pem_file(&public_key_file, LineEnding::default())
                .context("Error writing public key")?;
        }
        let private_key = RsaPrivateKey::read_pkcs8_pem_file(&private_key_file)
            .context("Error reading private key")?;
        let public_key = RsaPublicKey::read_public_key_pem_file(&public_key_file)
            .context("Error reading public key")?;
    
        Ok(Self{
            private: private_key,
            public: public_key
        })
    }
}

lazy_static! {
    static ref KEY_PAIR: RsaKeyPair = RsaKeyPair::read_or_generate("data/keys").unwrap();
}

impl Claims {
    fn parse(
        token: &str
    ) -> HttpResult<Self> {
        let key = KEY_PAIR.public.to_public_key_pem(LineEnding::default())
            .context("Error parsing key")?;
        let key = DecodingKey::from_rsa_pem(key.as_bytes())
            .context("Error parsing key")?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["api", "refresh"]);
        Ok(
            jsonwebtoken::decode(token, &key, &validation)
                .context("Error parsing token")?
                .claims
        )
    }
}

impl TokenPair {
    pub async fn new(
        pool: &PgPool,
        info: DeviceInfo,
        user_id: Uuid
    ) -> HttpResult<TokenPair> {
        let key = KEY_PAIR.private.to_pkcs8_pem(LineEnding::default())
            .context("Error parsing key")?;
        let key = EncodingKey::from_rsa_pem(key.as_bytes())
            .context("Error parsing key")?;

        let jti = Uuid::new_v4();
        let header = Header::new(Algorithm::RS256);

        let now = OffsetDateTime::now_utc();
        let iat = now.unix_timestamp() as usize;
        let access_exp = (now + ACCESS_LIFE_TIME).unix_timestamp() as usize;
        let refresh_exp = (now + REFRESH_LIFE_TIME).unix_timestamp() as usize;

        let access_claims = Claims {
            jti,
            aud: "api".to_string(),
            user_id: user_id,
            exp: access_exp,
            iat
        };

        let refresh_claims = Claims {
            jti,
            aud: "refresh".to_string(),
            user_id: user_id,
            exp: refresh_exp,
            iat
        };

        let access_token = jsonwebtoken::encode(&header, &access_claims, &key)
            .context("Error encoding access token")?;
        let refresh_token = jsonwebtoken::encode(&header, &refresh_claims, &key)
            .context("Error encoding refresh token")?;

        log::warn!("CREATING NEW USER SESSION\nid: {}\nuser_id: {}", jti, user_id);

        sqlx::query!(
            r#"
            INSERT INTO user_session (
                user_id,
                session_id,
                user_ip,
                user_agent,
                user_country,
                user_city
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )
            "#,
            user_id,
            jti,
            info.ip,
            info.os,
            info.country,
            info.city
        )
            .execute(pool)
            .await?;

        Ok(TokenPair {
            access: access_token,
            refresh: refresh_token
        })
    }

    pub async fn delete(
        pool: &PgPool,
        user_id: Uuid,
        session_id: Uuid
    ) -> HttpResult<()> {
        log::warn!("DELETING USER SESSION\nid: {}", session_id);

        sqlx::query!(
            r#"
            DELETE FROM user_session
            WHERE user_id = $1
            AND session_id = $2
            "#,
            user_id,
            session_id
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn refresh(
        pool: &PgPool,
        refresh_token: &str,
        info: DeviceInfo
    ) -> HttpResult<TokenPair> {
        let claims = Claims::parse(refresh_token)?;

        if sqlx::query!(
            r#"
            SELECT COUNT(1)
            FROM user_session
            WHERE session_id = $1
            "#,
            claims.jti
        )
        .fetch_one(pool)
        .await?
        .count != Some(1) {
            return Err(HttpError::Unauthorized);
        }

        TokenPair::delete(pool, claims.user_id, claims.jti).await?;
        Ok(TokenPair::new(pool, info, claims.user_id).await?)
    }
}

impl AuthUser {
    async fn from_authorization(
        pool: &PgPool,
        auth_header: &HeaderValue
    ) -> HttpResult<Self> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::error!("Can not convert header to string");
            HttpError::Unauthorized
        })?;

        if !auth_header.starts_with(PREFIX) {
            log::error!("Header does not start with `Bearer `");
            return Err(HttpError::Unauthorized);
        }

        let token = &auth_header[PREFIX.len()..];

        let claims = Claims::parse(token)
            .map_err(|_| {
                HttpError::Unauthorized
            })?;
        
        if (claims.exp as i64) < OffsetDateTime::now_utc().unix_timestamp() {
            log::info!("Token expired");
            return Err(HttpError::Unauthorized);
        }

        if sqlx::query!(
            r#"
            SELECT COUNT(1)
            FROM user_session
            WHERE session_id = $1
            "#,
            claims.jti
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
            WHERE session_id = $1
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
    type Rejection = HttpError;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let state: Extension<Arc<AppState>> = Extension::from_request_parts(req, _s)
            .await
            .expect("BUG: AppState was not added as an extension");
        
        let auth_header = req
            .headers
            .get(AUTHORIZATION)
            .ok_or(HttpError::Unauthorized)?;

        AuthUser::from_authorization(&state.pool, auth_header).await
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for MaybeAuthUser
where
    B: Send + Sync
{
    type Rejection = HttpError;

    async fn from_request_parts(req: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
        let state: Extension<Arc<AppState>> = Extension::from_request_parts(req, _s)
            .await
            .expect("BUG: AppState was not added as an extension");

        match req.headers.get(AUTHORIZATION) {
            Some(h) => Ok(MaybeAuthUser(AuthUser::from_authorization(&state.pool, h).await.ok())),
            None => Ok(MaybeAuthUser(None))
        }
    }
}