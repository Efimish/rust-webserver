use anyhow::anyhow;
use lazy_static::lazy_static;
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
use crate::http::{HttpError, HttpResult};

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

pub async fn hash_password(password: String) -> HttpResult<String> {
    Ok(tokio::task::spawn_blocking(move || -> HttpResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        ARGON2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                HttpError::Anyhow(
                    anyhow!("Error hashing password: {}", e)
                )
            })
            .map(|v| {
                v.to_string()
            })
    })
        .await
        .map_err(|e| {
            HttpError::Anyhow(
                anyhow!("Error hashing password: {}", e)
            )
        })??
    )
}

pub async fn verify_password(password: String, password_hash: String) -> HttpResult<()> {
    Ok(tokio::task::spawn_blocking(move || -> HttpResult<()> {
        let password_hash = PasswordHash::new(&password_hash)
            .map_err(|e| {
                HttpError::Anyhow(
                    anyhow!("Error getting password hash {}", e)
                )
            })?;
        ARGON2.verify_password(password.as_bytes(), &password_hash)
            .map_err(|_| {
                HttpError::BadRequest
            })
    })
        .await
        .map_err(|e| {
            HttpError::Anyhow(
                anyhow!("Error hashing password: {}", e)
            )
        })??
    )
}