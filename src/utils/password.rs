use anyhow::anyhow;
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
use crate::utils::Error;

fn new_argon2() -> Result<Argon2<'static>, Error> {
    Ok(Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            2_u32.pow(15),
            2,
            1,
            Some(32)
        )
            .map_err(|e| {
                Error::Anyhow(
                    anyhow!("Error creating argon2 params: {}", e)
                )
            })?
    ))
}

pub async fn hash_password(password: String) -> Result<String, Error> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = new_argon2()?;
        argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                Error::Anyhow(
                    anyhow!("Error hashing password: {}", e)
                )
            })
            .map(|v| {
                v.to_string()
            })
    })
        .await
        .map_err(|e| {
            Error::Anyhow(
                anyhow!("Error hashing password: {}", e)
            )
        })??
    )
}

pub async fn verify_password(password: String, password_hash: String) -> Result<(), Error> {
    Ok(tokio::task::spawn_blocking(move || -> Result<(), Error> {
        let argon2 = new_argon2()?;
        let password_hash = PasswordHash::new(&password_hash)
            .map_err(|e| {
                Error::Anyhow(
                    anyhow!("Error getting password hash {}", e)
                )
            })?;
        argon2.verify_password(password.as_bytes(), &password_hash)
            .map_err(|_| {
                Error::BadRequest
            })
    })
        .await
        .map_err(|e| {
            Error::Anyhow(
                anyhow!("Error hashing password: {}", e)
            )
        })??
    )
}