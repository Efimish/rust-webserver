use sqlx::PgPool;

use crate::{
    http::{AuthUser, RequestInfo, HttpError, HttpResult, HttpContext},
    models::{
        database_models::User,
        http_models::{AuthResponse, LoginBody, RefreshBody, RegisterBody},
    },
    utils::{
        password::{hash_password, verify_password},
        tokens::{Claims, TokenPair},
    },
};

async fn username_exists(pool: &PgPool, username: &str) -> HttpResult<bool> {
    let exists = sqlx::query!(
        r#"
        SELECT COUNT(1) FROM "user"
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await?
    .count
        == Some(1);
    Ok(exists)
}

async fn email_exists(pool: &PgPool, email: &str) -> HttpResult<bool> {
    let exists = sqlx::query!(
        r#"
        SELECT COUNT(1) FROM "user"
        WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await?
    .count
        == Some(1);
    Ok(exists)
}

pub async fn register(
    ctx: &HttpContext,
    body: RegisterBody,
    info: RequestInfo,
) -> HttpResult<AuthResponse> {
    // easier to keep everything in lowercase to avoid case insensitive checks
    let username = body.username.to_lowercase();
    let email = body.email.to_lowercase();
    if username_exists(&ctx.pool, &username).await? {
        // Username taken error
        return Err(HttpError::bad_request("Username is already taken"));
    }
    if email_exists(&ctx.pool, &email).await? {
        // Email taken error
        return Err(HttpError::bad_request("Email is already taken"));
    }
    let password_hash = hash_password(body.password).await?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO "user" (
            "username",
            "email",
            "password_hash",
            "display_name"
        ) VALUES ($1, $2, $3, $1)
        RETURNING "id", "username", "display_name", "avatar", "status", NULL::TIMESTAMPTZ AS "online"
        "#, username, email, password_hash
    )
    .fetch_one(&ctx.pool)
    .await?;

    let tokens = TokenPair::new(user.id).await?;

    let info = info.fetch_location(&ctx.client).await?;

    sqlx::query!(
        r#"
        INSERT INTO user_session (
            "id",
            "user_id",
            "user_ip",
            "user_agent",
            "user_country",
            "user_city"
        ) VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        tokens.id,
        user.id,
        info.ip,
        info.agent,
        info.country,
        info.city
    )
    .execute(&ctx.pool)
    .await?;

    Ok(AuthResponse { user, tokens })
}

pub async fn login(
    ctx: &HttpContext,
    body: LoginBody,
    info: RequestInfo,
) -> HttpResult<AuthResponse> {
    let username = body.username.to_lowercase();
    let password_hash = sqlx::query!(
        r#"
        SELECT "password_hash" FROM "user"
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(&ctx.pool)
    .await?
    // Username or password is wrong
    .ok_or(HttpError::bad_request("Username or password is wrong"))?
    .password_hash;

    verify_password(body.password, password_hash)
        .await
        // Username or password is wrong
        .map_err(|_| HttpError::bad_request("Username or password is wrong"))?;

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT "id", "username", "display_name", "avatar", "status", NULL::TIMESTAMPTZ AS "online"
        FROM "user"
        WHERE username = $1
        "#,
        body.username
    )
    .fetch_one(&ctx.pool)
    .await?;

    let tokens = TokenPair::new(user.id).await?;

    let info = info.fetch_location(&ctx.client).await?;

    sqlx::query!(
        r#"
        INSERT INTO user_session (
            "id",
            "user_id",
            "user_ip",
            "user_agent",
            "user_country",
            "user_city"
        ) VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        tokens.id,
        user.id,
        info.ip,
        info.agent,
        info.country,
        info.city
    )
    .execute(&ctx.pool)
    .await?;

    Ok(AuthResponse { user, tokens })
}

pub async fn refresh(
    ctx: &HttpContext,
    body: RefreshBody,
    info: RequestInfo,
) -> HttpResult<TokenPair> {
    let claims = Claims::parse(&body.refresh_token)?;

    let tokens = TokenPair::new(claims.user_id).await?;

    let info = info.fetch_location(&ctx.client).await?;

    sqlx::query!(
        r#"
        UPDATE user_session
        SET
            "id" = $2,
            "user_agent" = $3,
            "user_ip" = $4,
            "user_country" = $5,
            "user_city" = $6,
            "last_active" = NOW()
        WHERE "id" = $1
        "#,
        claims.jti,
        tokens.id,
        info.agent,
        info.ip,
        info.country,
        info.city
    )
    .execute(&ctx.pool)
    .await?;

    Ok(tokens)
}

pub async fn logout(ctx: &HttpContext, user: AuthUser) -> HttpResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM "user_session"
        WHERE "id" = $1
        "#,
        user.session_id
    )
    .execute(&ctx.pool)
    .await?;

    Ok(())
}