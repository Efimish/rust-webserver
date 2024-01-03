use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::utils::Error;

pub async fn add_user_session(
    pool: &PgPool,
    user_id: Uuid,
    session_id: Uuid,
    user_ip: &str,
    user_agent: &str,
    user_location: &str
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_session (
            user_id,
            session_id,
            user_ip,
            user_agent,
            user_location
        ) VALUES (
            $1, $2, $3, $4, $5
        )
        "#,
        user_id,
        session_id,
        user_ip,
        user_agent,
        user_location
    )
        .execute(pool)
        .await
        .map_err(|e| {
            Error::Sqlx(e)
        })
        .map(|_| {
            ()
        })
}

pub async fn remove_user_session(
    pool: &PgPool,
    session_id: Uuid
) -> Result<(), Error> {
    sqlx::query!(
        r#"DELETE FROM user_session WHERE session_id = $1"#,
        session_id
    )
        .execute(pool)
        .await
        .map_err(|e| {
            Error::Sqlx(e)
        })
        .map(|_| {
            ()
        })
}