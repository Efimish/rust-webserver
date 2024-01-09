use std::{net::SocketAddr, sync::Arc};
use axum::{Router, routing::post, http::HeaderMap, Json, extract::ConnectInfo, Extension};
use serde::Deserialize;
use crate::{AppState, utils::{AuthUser, MaybeAuthUser, TokenPair, RequestInfo, hash_password, verify_password, Error}};

pub fn router() -> Router {
    Router::new()
        .route(
            "/login",
            post(login)
        )
        .route(
            "/register",
            post(register)
        )
        .route(
            "/refresh",
            post(refresh)
        )
        .route(
            "/logout",
            post(logout)
        )
}

#[derive(Deserialize)]
struct LoginBody {
    username: String,
    password: String
}

async fn login(
    Extension(state): Extension<Arc<AppState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    user: MaybeAuthUser,
    body: Json<LoginBody>
) -> Result<Json<TokenPair>, Error> {
    let user_ip = connect_info.to_string();
    let user_ip = user_ip.split_once(":").unwrap().0.to_string();
    let user_agent = headers["user-agent"].to_str().unwrap();
    let user_agent = RequestInfo::get(&user_ip, user_agent).await?;
    let user_location = format!("{}, {}", user_agent.country, user_agent.city);
    let user_agent = user_agent.os;

    let username = body.username.to_lowercase();
    let found_user = sqlx::query!(
        r#"SELECT * FROM "user" WHERE username = $1"#,
        username)
        .fetch_one(&state.pool)
        .await;
    if found_user.is_err() {
        // return Err("Wrong username or password!");
        return Err(Error::BadRequest);
    }
    let found_user = found_user.unwrap();

    let user_id = found_user.user_id;
    let password_hash = found_user.password_hash;

    // let is_password_correct = verify_password(body.password.clone(), password_hash).await.is_ok();
    // if !is_password_correct {
    //     return (StatusCode::NOT_FOUND, Err("Wrong username or password!"));
    // }
    verify_password(body.password.clone(), password_hash).await?;

    println!("----- Logging in -----\nUsername: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.password, user_ip, user_agent);

    if let Some(u) = user.user_id() {
        TokenPair::delete(&state.pool, u).await;
    }
    let tokens = TokenPair::new(&state.pool, &state.keys.private,
        user_id, user_ip, user_agent, user_location).await;

    Ok(Json(tokens))
}

#[derive(Deserialize)]
struct RegisterBody {
    username: String,
    email: String,
    password: String
}

async fn register(
    Extension(state): Extension<Arc<AppState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Json<RegisterBody>
) -> Result<Json<TokenPair>, Error> {
    let user_ip = connect_info.to_string();
    let user_ip = user_ip.split_once(":").unwrap().0.to_string();
    let user_agent = headers["user-agent"].to_str().unwrap();
    let user_agent = RequestInfo::get(&user_ip, user_agent).await?;
    let user_location = format!("{}, {}", user_agent.country, user_agent.city);
    let user_agent = user_agent.os;

    let username = body.username.to_lowercase();
    let email = body.email.to_lowercase();
    let password_hash = hash_password(body.password.clone()).await?;
    println!("----- Registering -----\nUsername: {}\nEmail: {}\nPassword: {}\nIp: {}\nOS: {}", body.username, body.email, body.password, user_ip, user_agent);

    let same_username_exists = sqlx::query!(
        r#"SELECT COUNT(1) FROM "user"
        WHERE username = $1"#,
        username)
        .fetch_one(&state.pool)
        .await
        .unwrap()
        .count
        .unwrap() == 1;

    if same_username_exists {
        // return Err("User with this username already exists!");
        return Err(Error::BadRequest);
    }

    let user_id = sqlx::query!(
        r#"INSERT INTO "user" (
            username, email, password_hash
        ) VALUES (
            $1, $2, $3
        ) RETURNING user_id"#,
        username, email, password_hash)
        .fetch_one(&state.pool)
        .await
        .unwrap()
        .user_id;

    let tokens = TokenPair::new(&state.pool, &state.keys.private,
        user_id, user_ip, user_agent, user_location).await;
    
    Ok(Json(tokens))
}

#[derive(Deserialize)]
struct RefreshBody {
    #[serde(rename = "refreshToken")]
    refresh_token: String
}

async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    ConnectInfo(connect_info): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Json<RefreshBody>
) -> Result<Json<TokenPair>, Error> {
    let user_ip = connect_info.to_string();
    let user_ip = user_ip.split_once(":").unwrap().0.to_string();
    let user_agent = headers["user-agent"].to_str().unwrap();
    let user_agent = RequestInfo::get(&user_ip, user_agent).await?;
    let user_location = format!("{}, {}", user_agent.country, user_agent.city);
    let user_agent = user_agent.os;

    println!("----- Refreshing tokens -----\nIp: {}\nOS: {}", user_ip, user_agent);
    let tokens = TokenPair::refresh(&state.pool, &state.keys.private, &state.keys.public,
        &body.refresh_token, user_ip, user_agent, user_location).await;
    tokens.map(|t| Json(t))
}

async fn logout(
    Extension(state): Extension<Arc<AppState>>,
    user: AuthUser
) {
    TokenPair::delete(&state.pool, user.session_id).await;
}