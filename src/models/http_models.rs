//! # HTTP models
//! Used to represent Json bodies and other stuff http related.
//! Models are located in different files to separate them logically, but then all of them are re-exported

mod health;
pub use health::*;

mod auth;
pub use auth::*;

mod user;
pub use user::*;