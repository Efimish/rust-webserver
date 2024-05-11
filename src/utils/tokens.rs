//! # JsonWebToken creation and validation
//! Tokens are used to authenticate users.
//! There are two tokens: `access` and `refresh`.
//! `access` is used to access protected routes.
//! `refresh` is used to generate a new pair when `access` token expires.
//! Tokens are generated when user logs in.
//! To sign tokens, RSA keys are used.
//! Keys are taken from [Keys][super::keys] module.

use anyhow::Context;
use rsa::pkcs8::{
    EncodePrivateKey,
    EncodePublicKey,
    LineEnding
};
use jsonwebtoken::{EncodingKey, DecodingKey, Validation, Algorithm, Header};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use super::keys::KEY_PAIR;
use crate::http::HttpResult;

/// Tokens are sent to user as a pair.
/// Later, refresh token can be used to get a new pair.
/// Refresh token lives longer, but is used less often.
#[derive(Serialize)]
pub struct TokenPair {
    pub access: String,
    pub refresh: String,
    /// Session id
    #[serde(skip)]
    pub id: Uuid
}

/// Claims are parts of the token.
/// They are used to store information about the user and token itself
#[derive(Serialize, Deserialize)]
pub struct Claims {
    /// Token id
    pub jti: Uuid,
    /// Audience (what the token is intended for)\
    /// `"api"` or `"refresh"`
    pub aud: String,
    /// User id (to identify, whose token is this)
    pub user_id: Uuid,
    /// Expiration time (unix timestamp)
    pub exp: i64,
    /// Issued at (created at) (unix timestamp)
    pub iat: i64
}

const ACCESS_LIFE_TIME: Duration = Duration::minutes(10);
const REFRESH_LIFE_TIME: Duration = Duration::days(30);

lazy_static! {
    static ref ENCODING_KEY: EncodingKey = {
        let key = KEY_PAIR.private.to_pkcs8_pem(LineEnding::default()).unwrap();
        EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
    };
    static ref DECODING_KEY: DecodingKey = {
        let key = KEY_PAIR.public.to_public_key_pem(LineEnding::default()).unwrap();
        DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
    };
    static ref HEADER: Header = Header::new(Algorithm::RS256);
    static ref VALIDATION: Validation = {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["api", "refresh"]);
        validation
    };
}

impl Claims {
    /// Try to parse token string into valid claims
    pub fn parse(token: &str) -> HttpResult<Self> {
        Ok(
            jsonwebtoken::decode(token, &DECODING_KEY, &VALIDATION)
            .context("failed to parse token")?
            .claims
        )
    }
}

impl TokenPair {
    pub async fn new(user_id: Uuid) -> HttpResult<TokenPair> {
        let jti = Uuid::new_v4();

        let now = OffsetDateTime::now_utc();
        let iat = now.unix_timestamp();
        let access_exp = (now + ACCESS_LIFE_TIME).unix_timestamp();
        let refresh_exp = (now + REFRESH_LIFE_TIME).unix_timestamp();

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

        let access_token = jsonwebtoken::encode(&HEADER, &access_claims, &ENCODING_KEY)
            .context("failed to encode access token")?;
        let refresh_token = jsonwebtoken::encode(&HEADER, &refresh_claims, &ENCODING_KEY)
            .context("failed to encode refresh token")?;

        log::debug!("created new token pair\nid: {}\nuser_id: {}", jti, user_id);

        Ok(TokenPair {
            access: access_token,
            refresh: refresh_token,
            id: jti
        })
    }
}

// old code for instantly inserting tokens into database
// should be moved out of here

        // sqlx::query!(
        //     r#"
        //     INSERT INTO user_session (
        //         id,
        //         user_id,
        //         user_ip,
        //         user_agent,
        //         user_country,
        //         user_city
        //     ) VALUES (
        //         $1, $2, $3, $4, $5, $6
        //     )
        //     "#,
        //     jti,
        //     user_id,
        //     info.ip,
        //     info.os,
        //     info.country,
        //     info.city
        // )
        //     .execute(pool)
        //     .await?;

// pub async fn delete(
//     pool: &PgPool,
//     user_id: Uuid,
//     session_id: Uuid
// ) -> HttpResult<()> {
//     log::warn!("DELETING USER SESSION\nid: {}", session_id);

//     sqlx::query!(
//         r#"
//         DELETE FROM user_session
//         WHERE id = $1
//         AND user_id = $2
//         "#,
//         session_id,
//         user_id
//     )
//         .execute(pool)
//         .await?;

//     Ok(())
// }

// pub async fn refresh(
//     pool: &PgPool,
//     refresh_token: &str,
//     info: DeviceInfo
// ) -> HttpResult<TokenPair> {
//     let claims = Claims::parse(refresh_token)?;

//     if sqlx::query!(
//         r#"
//         SELECT COUNT(1)
//         FROM user_session
//         WHERE id = $1
//         "#,
//         claims.jti
//     )
//     .fetch_one(pool)
//     .await?
//     .count != Some(1) {
//         return Err(HttpError::Unauthorized);
//     }

//     TokenPair::delete(pool, claims.user_id, claims.jti).await?;
//     Ok(TokenPair::new(pool, info, claims.user_id).await?)
// }