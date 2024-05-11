//! # Models
//! Module containing models used across the application and custom types which are used in these models.

mod timestamptz;
pub use timestamptz::{Timestamptz, TimestamptzOption};

pub mod database_models;
pub mod http_models;