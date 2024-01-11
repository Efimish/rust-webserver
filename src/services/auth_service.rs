use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::utils::ReqResult;
use crate::models::session_model::BaseSession;

pub async fn add_user_session(
    pool: &PgPool,
    session: BaseSession
) -> ReqResult<()> {
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
        session.user_id,
        session.session_id,
        session.user_ip,
        session.user_agent,
        session.user_country,
        session.user_city
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn remove_user_session(
    pool: &PgPool,
    session_id: Uuid
) -> ReqResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM user_session
        WHERE session_id = $1
        "#,
        session_id
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_session(
    pool: &PgPool,
    session_id: Uuid
) -> ReqResult<bool> {
    Ok(sqlx::query!(
        r#"
        SELECT COUNT(1)
        FROM user_session
        WHERE session_id = $1
        "#,
        session_id
    )
        .fetch_one(pool)
        .await?
        .count == Some(1)
    )
}