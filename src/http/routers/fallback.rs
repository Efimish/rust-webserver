use axum::{body::Body, extract::Request};
use crate::http::HttpError;

pub async fn handler_404(
    req: Request<Body>,
) -> HttpError {
    let parts = req.into_parts().0;

    let path = parts.uri.path();
    let query = parts.uri.query()
        .map(|q| format!("?{}", q))
        .unwrap_or_default();
    let method = parts.method.to_string();
    let message = format!("Cannot {} {}{}", method, path, query);

    HttpError::not_found(message)
}