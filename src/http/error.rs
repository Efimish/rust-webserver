use axum::body::Body;
use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;

pub type HttpResult<T> = Result<T, HttpError>;

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    /// Return `400 Bad Request`
    #[error("bad request")]
    BadRequest,

    /// Return `400 Bad Request` on validation error
    #[error("bad request")]
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
    NotFound,

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
    #[allow(unused)]
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
            Self::BadRequest | Self::Validator(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Redis(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::UnprocessableEntity { ref errors } => {
                #[derive(Serialize)]
                struct Error {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>
                }

                return (
                    self.status_code(),
                    Json(Error { errors: errors.clone() })
                ).into_response();
            }
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    self.to_string(),
                ).into_response();
            },
            Self::Sqlx(ref e) => {
                log::error!("SQLx error: {:?}", e);
            },
            Self::Redis(ref e) => {
                log::error!("Redis error: {:?}", e);
            }
            Self::Anyhow(ref e) => {
                log::error!("Generic error: {:?}", e);
            },
            Self::Validator(ref errors) => {
                log::error!("Validation error: {:?}", errors);
                #[derive(Serialize)]
                struct Error<'a> {
                    message: &'a str,
                    errors: HashMap<&'a str, Vec<Option<Cow<'a, str>>>>
                }
                let message = "Validation error";
                let errors: HashMap<&str, Vec<Option<Cow<str>>>> = errors.clone()
                    .field_errors()
                    .iter()
                    .map(|(&code, &e)| {
                        (code, e.iter().map(|er| er.message.clone()).collect())
                    })
                    .collect();
                return (
                    self.status_code(),
                    Json(Error { message, errors })
                ).into_response();
            }
            _ => ()
        }
        #[derive(Serialize)]
        struct ResponseError {
            message: String
        }
        (
            self.status_code(),
            Json(
                ResponseError {
                    message: self.to_string()
                }
            )
        ).into_response()
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