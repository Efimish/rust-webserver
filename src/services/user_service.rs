use sqlx::postgres::PgPool;
use uuid::Uuid;
use regex::Regex;
use crate::utils::Error;
use crate::models::user_model::BaseUser;

async fn verify_username(
    pool: &PgPool,
    username: &str
) -> Result<(), Error> {
    let username_regex = Regex::new(r"^\w+$").unwrap();

    if username.len() < 3
    || username.len() > 24
    || username != username.to_lowercase()
    || username_regex.is_match(username) {
        return Err(Error::BadRequest);
    }
    if sqlx::query!(
        r#"
        SELECT COUNT(1) FROM "user"
        WHERE username = $1
        "#,
        username
    )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            Error::Sqlx(e)
        })?
        .count == Some(1) {
            return Err(Error::BadRequest);
        };
    Ok(())
}

pub async fn add_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<Uuid, Error> {
    verify_username(pool, username).await?;

    sqlx::query!(
        r#"
        INSERT INTO "user" (
            username,
            email,
            password_hash
        ) VALUES (
            $1, $2, $3
        ) RETURNING user_id
        "#,
        username,
        email,
        password
    )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            Error::Sqlx(e)
        })
        .map(|v| {
            v.user_id
        })
}

pub async fn get_users(
    pool: &PgPool
) -> Result<Vec<BaseUser>, Error> {
    sqlx::query_as!(
        BaseUser,
        r#"
        SELECT user_id, username FROM "user"
        "#
    )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            Error::Sqlx(e)
        })
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username: &str
) -> Result<BaseUser, Error> {
    verify_username(pool, username).await?;

    sqlx::query_as!(
        BaseUser,
        r#"
        SELECT user_id, username FROM "user"
        WHERE username = $1
        "#,
        username
    )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            Error::Sqlx(e)
        })
}

pub async fn delete_user(
    pool: &PgPool,
    username: &str
) -> Result<(), Error> {
    verify_username(pool, username).await?;

    sqlx::query!(
        r#"
        DELETE FROM "user"
        WHERE username = $1
        "#,
        username
    )
        .execute(pool)
        .await
        .map_err(|e| {
            Error::Sqlx(e)
        })?;
    
    Ok(())
}