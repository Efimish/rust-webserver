use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::utils::{
    ReqResult,
    Error
};
use crate::models::{
    chat_model::DBChat,
    message_model::DBMessage
};

pub async fn get_chat_messages(
    pool: &PgPool,
    user_id: Uuid,
    chat_id: Uuid
) -> ReqResult<Vec<DBMessage>> {
    if sqlx::query!(
        r#"
        SELECT COUNT(1) FROM chat_user
        WHERE user_id = $1
        AND chat_id = $2
        "#,
        user_id,
        chat_id
    )
        .fetch_one(pool)
        .await?
        .count == Some(0) {
        return Err(Error::BadRequest);
    }
    Ok(sqlx::query_as!(
        DBMessage,
        r#"
        SELECT m.*
        FROM chat c
        JOIN message m on m.chat_id = c.chat_id
        WHERE c.chat_id = $1
        "#,
        chat_id
    )
        .fetch_all(pool)
        .await?
    )
}

pub async fn add_chat(
    pool: &PgPool,
    user_id: Uuid,
    chat_name: String
) -> ReqResult<()> {
    let chat_id = sqlx::query!(
        r#"
        INSERT INTO chat (
            chat_name
        ) VALUES (
            $1
        ) RETURNING chat_id
        "#,
        chat_name
    )
        .fetch_one(pool)
        .await?
        .chat_id;
    sqlx::query!(
        r#"
        INSERT INTO chat_user (
            chat_id,
            user_id
        ) VALUES (
            $1, $2
        )
        "#,
        chat_id,
        user_id
    )
        .execute(pool)
        .await?;
    Ok(())
}

#[allow(unused)]
pub async fn delete_chat(
    pool: &PgPool,
    chat_id: Uuid
) -> ReqResult<()> {    
    sqlx::query!(
        r#"
        DELETE FROM chat
        WHERE chat_id = $1
        "#,
        chat_id
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_my_chats(
    pool: &PgPool,
    user_id: Uuid
) -> ReqResult<Vec<DBChat>> {
    Ok(sqlx::query_as!(
        DBChat,
        r#"
        SELECT c.* FROM "user" u
        JOIN chat_user cu ON cu.user_id = u.user_id
        JOIN chat c ON c.chat_id = cu.chat_id
        WHERE u.user_id = $1
        "#,
        user_id
    )
        .fetch_all(pool)
        .await?
    )
}

pub async fn send_chat_message(
    pool: &PgPool,
    user_id: Uuid,
    chat_id: Uuid,
    context: String
) -> ReqResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO message (
            chat_id,
            sender_id,
            context
        ) VALUES (
            $1, $2, $3
        )
        "#,
        chat_id,
        user_id,
        context
    )
        .execute(pool)
        .await?;
    Ok(())
}