use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::utils::{Error, ReqResult};
use crate::models::user_model::{
    AddUser,
    BaseUser
};

async fn verify_available(
    pool: &PgPool,
    username: &str
) -> ReqResult<bool> {
    Ok(sqlx::query!(
        r#"
        SELECT COUNT(1) FROM "user"
        WHERE username = $1
        "#,
        username
    )
        .fetch_one(pool)
        .await?
        .count == Some(0)
    )
}

pub async fn get_user(
    pool: &PgPool,
    username: &str
) -> ReqResult<BaseUser> {
    // verify_spelling(username)?;
    Ok(sqlx::query_as!(
        BaseUser,
        r#"
        SELECT username, display_name, status FROM "user"
        WHERE username = $1
        "#,
        username
    )
        .fetch_one(pool)
        .await?
    )
}

pub async fn add_user(
    pool: &PgPool,
    user: AddUser
) -> ReqResult<Uuid> {
    if !verify_available(pool, &user.username).await? {
        return Err(Error::BadRequest);
    };

    Ok(sqlx::query!(
        r#"
        INSERT INTO "user" (
            username,
            email,
            password_hash
        ) VALUES (
            $1, $2, $3
        ) RETURNING user_id
        "#,
        user.username,
        user.email,
        user.password_hash
    )
        .fetch_one(pool)
        .await?
        .user_id
    )
}

pub async fn delete_user(
    pool: &PgPool,
    username: &str
) -> ReqResult<()> {
    // verify_spelling(username)?;
    if verify_available(pool, username).await? {
        return Err(Error::BadRequest);
    }
    
    sqlx::query!(
        r#"
        DELETE FROM "user"
        WHERE username = $1
        "#,
        username
    )
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn get_users(
    pool: &PgPool
) -> ReqResult<Vec<BaseUser>> {
    Ok(sqlx::query_as!(
        BaseUser,
        r#"
        SELECT username, display_name, status FROM "user"
        "#
    )
        .fetch_all(pool)
        .await?
    )
}