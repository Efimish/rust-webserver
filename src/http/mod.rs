mod error;
mod device_info;
mod auth_user;
mod timestampz;
mod password;
pub use error::{HttpError, HttpResult};
pub use device_info::DeviceInfo;
pub use auth_user::{AuthUser, MaybeAuthUser, TokenPair};
pub use timestampz::Timestampz;

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