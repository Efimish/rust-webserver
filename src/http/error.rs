//! Error type definition and implementation of `IntoResponse`

use axum::body::Body;
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;

/// # Result type wrapper
/// Just use it to type less.
pub type HttpResult<T> = Result<T, HttpError>;

/// # Error used for anything HTTP related
/// Main error type, it represents various errors that can occur during a request.\
/// Serializes as a JSON. Response example:
/// ```rust,ignore
/// {
///     message: "Username is already taken",
///     error: "Bad Request",
///     statusCode: 400
/// }
/// ```
#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    /// Return `400 Bad Request`
    #[error("bad request")]
    BadRequest(String),

    /// Return `400 Bad Request` on validation error
    #[error("validation failed")]
    Validator(#[from] validator::ValidationErrors),

    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    #[allow(unused)]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    #[allow(unused)]
    NotFound(String),

    /// Return `422 Unprocessable Entity`
    #[error("error in the request body")]
    #[allow(unused)]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    /// Automatically return `500 Internal Server Error` on a `sqlx::Error`.
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    /// Automatically return `500 Internal Server Error` on a `redis::RedisError`.
    #[error("an error occurred with the database")]
    Redis(#[from] redis::RedisError),

    /// Return `500 Internal Server Error` on a `anyhow::Error`.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl HttpError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn not_found(path: impl Into<String>) -> Self {
        Self::NotFound(path.into())
    }
    
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) | Self::Validator(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Redis(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error(&self) -> String {
        match self {
            Self::BadRequest(_) => "Bad Request",
            Self::Validator(_) => "Validation Error",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound(_) => "Not Found",
            Self::UnprocessableEntity { .. } => "Unprocessable Entity",
            Self::Sqlx(_) | Self::Redis(_) | Self::Anyhow(_) => "Internal Server Error"
        }.to_string()
    }

    fn message(&self) -> Option<String> {
        match self {
            Self::BadRequest(ref message) => Some(message.clone()),
            Self::NotFound(ref message) => Some(message.clone()),
            Self::Validator(ref errors) => {
                for &field_errors in errors.field_errors().values() {
                    for error in field_errors {
                        if let Some(message) = &error.message {
                            return Some(message.to_string());
                        }
                    }
                }
                None
            },
            Self::UnprocessableEntity { ref errors } => {
                if let Some(errors) = errors.values().next() {
                    if let Some(error) = errors.first() {
                        return Some(error.to_string());
                    }
                };
                None
            },
            _ => None
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseError {
    message: Option<String>,
    error: String,
    status_code: u16,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response<Body> {
        let status_code = self.status_code();
        let error = self.error();
        let message = self.message();

        let response = ResponseError { status_code: status_code.as_u16(), message, error };

        match self {
            Self::Unauthorized => {
                return (
                    status_code,
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    Json(response),
                ).into_response();
            }
            Self::Sqlx(ref e) => {
                log::error!("SQLx error: {:?}", e);
            }
            Self::Redis(ref e) => {
                log::error!("Redis error: {:?}", e);
            }
            Self::Anyhow(ref e) => {
                log::error!("Generic error: {:?}", e);
            }
            _ => ()
        }

        (status_code, Json(response)).into_response()
    }
}

pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> HttpError
    ) -> Result<T, HttpError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<HttpError>
{
    fn on_constraint(
            self,
            name: &str,
            map_err: impl FnOnce(Box<dyn DatabaseError>) -> HttpError
    ) -> HttpResult<T> {
        self.map_err(|e| match e.into() {
            HttpError::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}