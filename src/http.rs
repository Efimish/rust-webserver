//! # HTTP
//! Module containing http related stuff.

mod error;
mod extractors;
mod context;

pub use error::*;
pub use extractors::*;
pub use context::*;

pub mod routers;

// Some stuff I did not finish
trait HttpErrorContext<T> {
    fn http_context(self, error: HttpError) -> HttpResult<T>;
}

impl<T, E> HttpErrorContext<T> for Result<T, E> {
    fn http_context(self, error: HttpError) -> HttpResult<T> {
        self.map_err(|_| error)
    }
}

impl<T> HttpErrorContext<T> for Option<T> {
    fn http_context(self, error: HttpError) -> HttpResult<T> {
        self.ok_or(error)
    }
}
