use thiserror::Error;

use crate::domain::types::ApiRequest;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Responder experienced an error.")]
    ResponderMalfunction(#[from] ResponderError),
}

#[derive(Error, Debug)]
pub enum ResponderError {
    #[error(
        "Request {req:?} with {name:?} is not matching any response name."
    )]
    RequestNotMatching { name: String, req: ApiRequest },
    #[error("Data for `{0}` not found.")]
    NotFound(String),
    #[error("HTTP-Client experienced an error.")]
    HttpClientError(#[from] reqwest::Error),
}
