use serde::Deserialize;

use crate::ser;

/// Errors for [`HelixClient::req_get`] and similar functions.
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum ClientRequestError<RE: std::error::Error + Send + Sync + 'static> {
    /// request failed from reqwests side
    RequestError(RE),
    /// no pagination found
    NoPage,
    /// could not create request
    CreateRequestError(#[from] CreateRequestError),
    /// could not parse GET response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    ApiRequestGetError(#[from] ApiRequestGetError),
    /// could not parse PUT response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    ApiRequestPutError(#[from] ApiRequestPutError),
    /// could not parse POST response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    ApiRequestPostError(#[from] ApiRequestPostError),
    /// could not parse PATCH response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    ApiRequestPatchError(#[from] ApiRequestPatchError),
    /// could not parse DELETE response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    ApiRequestDeleteError(#[from] ApiRequestDeleteError),
    /// {0}
    Custom(std::borrow::Cow<'static, str>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApiRequestError {
    pub error: String,
    pub status: u16,
    pub message: String,
}

/// Could not create request
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum CreateRequestError {
    /// http crate returned an error
    HttpError(#[from] http::Error),
    /// serialization of body failed
    SerializeError(#[from] BodyError),
    /// could not assemble URI for request
    InvalidUri(#[from] InvalidUri),
    /// {0}
    Custom(std::borrow::Cow<'static, str>),
}

/// Errors that can happen when creating [`http::Uri`] for [`Request`]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum InvalidUri {
    /// URI could not be parsed
    UriParseError(#[from] http::uri::InvalidUri),
    /// could not serialize request to query
    QuerySerializeError(#[from] ser::Error),
}

/// Could not parse GET response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum ApiRequestGetError {
    /// helix returned error {status:?} - {error}: {message:?} when calling
    /// `GET {uri}`
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
    },
    /// could not parse response as utf8 when calling `GET {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `GET
    /// {2}` with response: {0:?}
    DeserializeError(String, #[source] serde_json::Error, http::Uri),
    // FIXME: Only used in webhooks parse_payload
    /// could not get URI for request
    InvalidUri(#[from] InvalidUri),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse PUT response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum ApiRequestPutError {
    /// helix returned error {status:?} - {error}: {message:?} when calling
    /// `PUT {uri}`
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
    },
    /// could not parse response as utf8 when calling `PUT {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `PUT
    /// {2}` with response: {0:?}
    DeserializeError(String, #[source] serde_json::Error, http::Uri),
}

/// Could not parse POST response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum ApiRequestPostError {
    /// helix returned error {status:?} - {error}: {message:?} when calling
    /// `POST {uri}` with a body
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent with POST
        body: Vec<u8>,
    },
    /// could not parse response as utf8 when calling `POST {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `POST
    /// {2}` with response: {0:?}
    DeserializeError(String, #[source] serde_json::Error, http::Uri),
}

/// helix returned error {status:?}: {message:?} when calling `PATCH {uri}` with
/// a body
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub struct ApiRequestPatchError {
    /// Status code of error, usually 400-499
    status: http::StatusCode,
    /// Error message from Twitch
    message: String,
    /// URI to the endpoint
    uri: http::Uri,
    /// Body sent with PATCH
    body: Vec<u8>,
}

/// Could not parse DELETE response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum ApiRequestDeleteError {
    /// helix returned error {status:?}- {error}: {message:?} when calling
    /// `DELETE {uri}`
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
    },
    /// could not parse response as utf8 when calling `DELETE {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
}

/// Errors that can happen when creating a body for
/// [`RequestPost`](RequestPost::Body) or [`RequestPatch`](RequestPatch::Body)
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum BodyError {
    /// could not serialize as json
    JsonError(#[from] serde_json::Error),
    /// could not serialize to query
    QuerySerializeError(#[from] ser::Error),
    /// uri is invalid
    InvalidUri(#[from] InvalidUri),
}
