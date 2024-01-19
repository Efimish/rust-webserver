mod error;
mod timestampz;
mod extractors;
mod password;
pub use error::{HttpError, HttpResult};
pub use timestampz::Timestampz;
pub use extractors::*;

mod routers;
pub use routers::{router, AppState};

trait HttpContext<T, E> {
    fn http_context(self, error: E) -> Result<T, E>;
}

impl<T, E> HttpContext<T, HttpError> for Result<T, E> {
    fn http_context(self, error: HttpError) -> Result<T, HttpError> {
        self.map_err(|_| error)
    }
}

impl<T> HttpContext<T, HttpError> for Option<T> {
    fn http_context(self, error: HttpError) -> Result<T, HttpError> {
        self.ok_or(error)
    }
}