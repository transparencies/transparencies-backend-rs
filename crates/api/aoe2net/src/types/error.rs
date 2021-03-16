use displaydoc::Display;
use thiserror::Error;

/// Error thrown from aoe2.net API endpoints
#[derive(Debug, Error, Display)]
pub enum ApiError {
    /// Generic Error thrown by the API
    Generic(String),
}
