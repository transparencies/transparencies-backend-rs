//! Error types of our library part
use std::num::ParseIntError;

use displaydoc::Display;
use thiserror::Error;

use crate::domain::types::{
    ApiRequest,
    GithubFileRequest,
};

/// Error type for the `MatchInfoProcessor`
#[derive(Error, Display, Debug)]
pub enum ProcessingError {
    /// No candidate for civilisation found.
    CivilisationError,
    /// Assembly of Matchinfo result failed. Result is empty.
    AssemblyError,
    /// Responder experienced an error.
    ResponderMalfunction(#[from] ResponderError),
    /// Parsing of Integer data failed
    ProcessIntParsingError(#[from] ParseIntError),
    /// Conversion to String failed.
    SerdeStringConversionError(#[from] serde_json::Error),
    /// Dividing by Zero is not allowed.
    DividingByZeroError,
    /// Haven't found a rating for player id: {0}
    LookupRatingNotFound(i64),
    /// Haven't found a leaderboard value for player id: {0}
    LeaderboardNotFound(i64),
    /// Haven't found a translation for {0}: {1}
    TranslationError(String, usize),
}

/// Error type for the `MatchInfoResponder`
#[derive(Error, Display, Debug)]
pub enum ResponderError {
    /// Request {req:?} with {name:?} is not matching any response name.
    RequestNotMatching {
        /// Request name
        name: String,
        /// Request itself
        req: ApiRequest,
    },
    /// Data for `{0}` not found.
    NotFound(String),
    /// HTTP-Client experienced an error.
    HttpClientError(#[from] reqwest::Error),
    /// Parsing of Integer data failed
    RespondIntParsingError(#[from] ParseIntError),
    /// Conversion to String failed.
    SerdeStringConversionError(#[from] serde_json::Error),
    /// Haven't found a translation for {0}: {1}
    TranslationError(String, usize),
    /// Couldn't get the value of the translation string: {0} at given index
    /// {1}
    TranslationPosError(String, usize),
    /// Couldn't get the value of the translation string, because it has
    /// already been moved.
    TranslationHasBeenMovedError,
}

/// Error type for a `FileRequest`
#[derive(Error, Display, Debug)]
pub enum FileRequestError {
    /// Request {req:?} with {name:?} is not matching any response name.
    RequestNotMatching {
        /// Request name
        name: String,
        /// Request itself
        req: GithubFileRequest,
    },
    /// HTTP-Client experienced an error.
    HttpClientError(#[from] reqwest::Error),
    /// JSON-Deserialisation failed.
    JsonDeserializationError(#[from] serde_json::Error),
    /// YAML-Deserialisation failed.
    YamlDeserializationError(#[from] serde_yaml::Error),
    /// URL parsing failed.
    UrlParsingError(#[from] url::ParseError),
}

/// Error type for an [`ApiRequest`]
#[derive(Error, Display, Debug)]
pub enum ApiRequestError {
    /// Request {req:?} with {name:?} is not matching any response name.
    RequestNotMatching {
        /// Request name
        name: String,
        /// Request itself
        req: ApiRequest,
    },
    /// HTTP-Client experienced an error.
    HttpClientError(#[from] reqwest::Error),
}

/// Error type for an [`ApiRequest`]
#[derive(Error, Display, Debug)]
pub enum TestCaseError {
    /// RON-Parsing failed.
    RonParsing(#[from] ron::de::Error),
    /// JSON-Parsing failed.
    JsonParsing(#[from] serde_json::Error),
    /// File failed to open.
    Io(#[from] std::io::Error),
}

/// Error type for the Indexing functionality
#[derive(Error, Display, Debug)]
pub enum IndexingError {
    /// Player {name:?} with Profile ID {profile_id:?} does already exist in
    /// the index at position {pos:?}, doublette is {doublette:?}.
    PlayerAlreadyExisting {
        /// Player name
        name: String,
        /// profile_id of the corresponding Player
        profile_id: String,
        /// Position in Vector
        pos: usize,
        /// Position of doublet
        doublet: usize,
    },
}
