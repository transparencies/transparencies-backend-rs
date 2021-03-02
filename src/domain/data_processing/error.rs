use thiserror::Error;

use crate::domain::types::{
    ApiRequest,
    GithubFileRequest,
};

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Assembly of Matchinfo result failed. Result is empty.")]
    AssemblyError,
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

#[derive(Error, Debug)]
pub enum FileRequestError {
    #[error(
        "Request {req:?} with {name:?} is not matching any response name."
    )]
    RequestNotMatching {
        name: String,
        req: GithubFileRequest,
    },
    #[error("HTTP-Client experienced an error.")]
    HttpClientError(#[from] reqwest::Error),
}

#[derive(Error, Debug)]
pub enum ApiRequestError {
    #[error(
        "Request {req:?} with {name:?} is not matching any response name."
    )]
    RequestNotMatching { name: String, req: ApiRequest },
    #[error("HTTP-Client experienced an error.")]
    HttpClientError(#[from] reqwest::Error),
}

#[derive(Error, Debug)]
pub enum IndexingError {
    #[error(
        "Player {name:?} with Profile ID {profile_id:?} does already exist in the index at position {pos:?}, doublette is {doublette:?}."
    )]
    PlayerAlreadyExisting {
        name: String,
        profile_id: String,
        pos: usize,
        doublette: usize,
    },
}
