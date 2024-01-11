use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::utils::{Error, ReqResult};
use crate::models::user_model::{
    FixedRegisterBody,
    BaseUser, FixedLoginBody
};

async fn find_user(
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
        .count == Some(1)
    )
}

pub async fn get_user_password(
    pool: &PgPool,
    username: &str
) -> ReqResult<FixedLoginBody> {
    if !find_user(pool, username).await? {
        return Err(Error::BadRequest);
    }
    Ok(sqlx::query_as!(
        FixedLoginBody,
        r#"
        SELECT user_id, password_hash
        FROM "user"
        WHERE username = $1
        "#,
        username
    )
        .fetch_one(pool)
        .await?
    )
}

pub async fn get_user(
    pool: &PgPool,
    username: &str
) -> ReqResult<BaseUser> {
    if !find_user(pool, username).await? {
        return Err(Error::BadRequest);
    }
    Ok(sqlx::query_as!(
        BaseUser,
        r#"
        SELECT username, display_name, status
        FROM "user"
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
    user: FixedRegisterBody
) -> ReqResult<Uuid> {
    if find_user(pool, &user.username).await? {
        return Err(Error::BadRequest);
    };

    Ok(sqlx::query!(
        r#"
        INSERT INTO "user" (
            username,
            email,
            password_hash,
            display_name
        ) VALUES (
            $1, $2, $3, $1
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
    if !find_user(pool, username).await? {
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