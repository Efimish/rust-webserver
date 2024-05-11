//! Custom extractor for validated JSON payloads.

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
    Json,
};
use validator::Validate;

use crate::http::HttpError;

/// # JSON body extractor
/// [axum] has a [built-in JSON extractor][axum::Json],
/// but it seems impossible to get an error message out of it.
/// Also, it does not validate the JSON payload.
/// Each struct using this must implement the [Validate] trait from the [validator] crate.
/// If you dont need the validation, simply derive the trait but do not add `#[validate(...)]` attributes on fields.
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    T: Validate,
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request(req: Request<Body>, _s: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, _s)
            .await
            .map_err(|e| HttpError::bad_request(e.body_text()))?;

        data.validate()?;

        Ok(Self(data))
    }
}
