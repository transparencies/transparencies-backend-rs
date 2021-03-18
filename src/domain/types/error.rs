//! Error types of our library part
use std::{
    borrow::Cow,
    num::ParseIntError,
};

use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

use crate::domain::types::{
    ApiRequest,
    GithubFileRequest,
};

#[derive(Error,
           displaydoc::Display,
           Debug,
           Serialize,
           Clone,
           PartialEq,
           Deserialize)]
pub enum ErrorMessageToFrontend {
    /// Generic error from the Responder: {0}
    GenericResponderError(Cow<'static, str>),
    /// Matchinfo processing failed: {0}
    HardFail(Cow<'static, str>),
    /// Matchinfo processing failed: {0}
    SoftFail(Cow<'static, str>),
    /// Rocover: {0}
    Recover(Cow<'static, str>),
}

/// Error type for the `MatchInfoProcessor`
#[derive(Error, displaydoc::Display, Debug)]
pub enum ProcessingError {
    /// No candidate for civilisation found.
    CivilisationError,
    /// Assembly of Matchinfo result failed. Result is empty.
    AssemblyError,
    /// Responder experienced an error: {0}
    ResponderMalfunction(#[from] ResponderError),
    /// Parsing of Integer data failed: {0}
    ProcessIntParsingError(#[from] ParseIntError),
    /// Conversion to String failed: {0}
    SerdeStringConversionError(#[from] serde_json::Error),
    /// Dividing by Zero is not allowed.
    DividingByZeroError,
    /// Haven't found a rating for player id: {0}
    LookupRatingNotFound(u64),
    /// Haven't found a leaderboard response for player id: {0}
    LeaderboardNotFound(u64),
    /// Haven't found a translation for {0}: {1}
    TranslationError(String, usize),
    /// Player id {0} is not ranked on that leaderboard.
    NotRankedLeaderboard(u64),
}

/// Error type for the `MatchInfoResponder`
#[derive(Error, displaydoc::Display, Debug)]
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
    /// HTTP-Client experienced an error: {0}
    HttpClient(#[from] reqwest::Error),
    /// Parsing of Integer data failed: {0}
    RespondIntParsingFailed(#[from] ParseIntError),
    /// Conversion to String failed: {0}
    SerdeStringConversion(#[from] serde_json::Error),
    /// Haven't found a translation for {0}: {1}
    TranslationFailed(String, usize),
    /** Couldn't get the value of the translation string: {0} at given index
     * {1}
     */
    TranslationPosError(String, usize),
    /** Couldn't get the value of the translation string, because it has
     * already been moved.
     */
    TranslationHasBeenMoved,
    /// Other ApiRequestError: {0}.
    OtherApiRequestError(#[from] ApiRequestError),
    /// Data for LastMatch not found, possible unrecorded player detected.
    LastMatchNotFound,
    /// Invalid id_type: {0}
    InvalidIdType(Cow<'static, str>),
    /// Invalid RequestType: {0}
    InvalidReqType(String),
    // /// Unsupported
    // NotSupported {
    //     /// Location where this was triggered
    //     location: &'static std::panic::Location<'static>,
    // },
    /// UUID parsing failed: {0}
    ParsingError(#[from] uuid::Error),
}

/// Error type for a `FileRequest`
#[derive(Error, displaydoc::Display, Debug)]
pub enum FileRequestError {
    /// Request {req:?} with {name:?} is not matching any response name.
    RequestNotMatching {
        /// Request name
        name: String,
        /// Request itself
        req: GithubFileRequest,
    },
    /// HTTP-Client experienced an error: {0}
    HttpClientError(#[from] reqwest::Error),
    /// JSON-Deserialisation failed: {0}
    JsonDeserializationError(#[from] serde_json::Error),
    /// YAML-Deserialisation failed: {0}
    YamlDeserializationError(#[from] serde_yaml::Error),
    /// URL parsing failed: {0}
    UrlParsingError(#[from] url::ParseError),
}

/// Error type for an [`ApiRequest`]
#[derive(Error, displaydoc::Display, Debug)]
pub enum ApiRequestError {
    /// Request {req:?} with {name:?} is not matching any response name.
    RequestNotMatching {
        /// Request name
        name: String,
        /// Request itself
        req: ApiRequest,
    },
    /// HTTP-Client experienced an error: {0}
    HttpClientError(#[from] reqwest::Error),
    /// ApiClient experienced an error: {0}
    ApiClientError(#[from]
                   api_client::error::ClientRequestError<reqwest::Error>),
    /// HTTP-Client error with status code: {0}
    HttpClientErrorWithStatusCode(http::StatusCode),
    /// Response NotFound: {root:?}/{endpoint:?} with {query:?}
    NotFoundResponse {
        root: String,
        endpoint: String,
        query: Vec<(String, String)>,
    },
}

/// Error type for an [`ApiRequest`]
#[derive(Error, displaydoc::Display, Debug)]
pub enum TestCaseError {
    /// RON-Parsing failed.
    RonParsing(#[from] ron::de::Error),
    /// JSON-Parsing failed.
    JsonParsing(#[from] serde_json::Error),
    /// File failed to open.
    Io(#[from] std::io::Error),
}

/// Error type for the Indexing functionality
#[derive(Error, displaydoc::Display, Debug)]
pub enum IndexingError {
    /** Player {name:?} with Profile ID {profile_id:?} does already exist in
     * the index at position {pos:?}, doublette is {doublet:?}.
     */
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
