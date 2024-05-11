//! # Argon2 password hashing and verification
//! Hasing passwords is a computationally intensive task,
//! so it is done inside a blocking thread.

use lazy_static::lazy_static;
use anyhow::{anyhow, Context};
use argon2::{
    Argon2,
    Algorithm,
    Version,
    Params,
    password_hash::{
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
        rand_core::OsRng
    }
};
use crate::http::HttpResult;

lazy_static! {
    static ref ARGON2: Argon2<'static> = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            2_u32.pow(15),
            2,
            1,
            Some(32)
        ).unwrap()
    );
}

// Those are called green threads

/// Hashes a password using Argon2.
/// It is computationally intensive,
/// so it will happen inside a blocking thread.
pub async fn hash_password(password: String) -> HttpResult<String> {
    tokio::task::spawn_blocking(move || -> HttpResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(ARGON2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("failed to hash password: {}", e))?
            .to_string())
    })
    .await.context("failed to hash password")?
}

/// Verifies a password using Argon2.
/// It is computationally intensive,
/// so it will happen inside a blocking thread.
pub async fn verify_password(password: String, password_hash: String) -> HttpResult<()> {
    tokio::task::spawn_blocking(move || -> HttpResult<()> {
        let password_hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow!("failed to get password hash {}", e))?;
        ARGON2.verify_password(password.as_bytes(), &password_hash)
            // .map_err(|e| match e {
            //     argon2::password_hash::Error::Password => HttpError::Unauthorized,
            //     _ => anyhow!("failed to verify password: {}", e).into(),
            // })
            .map_err(|e| anyhow!("failed to verify password: {}", e).into())
    })
    .await.context("failed to verify password")?
}