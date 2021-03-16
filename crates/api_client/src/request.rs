use crate::response::Response;

/// A request is an API endpoint
#[async_trait::async_trait]
pub trait Request: serde::Serialize {
    /// The path to the endpoint relative to the helix root. eg. `channels` for [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    const PATH: &'static str;
    /// Response type. twitch's response will  deserialize to this.
    type Response: serde::de::DeserializeOwned + PartialEq;
    // /// Defines layout of the url parameters.
    // fn query(&self) -> Result<String, ser::Error> {
    //     ser::to_string(&self)
    // }
    // /// Returns full URI for the request, including query parameters.
    // fn get_uri(&self) -> Result<http::Uri, InvalidUri> {
    //     http::Uri::from_str(&format!(
    //         "{}{}?{}",
    //         crate::TWITCH_HELIX_URL,
    //         <Self as Request>::PATH,
    //         self.query()?
    //     ))
    //     .map_err(Into::into)
    // }
    // /// Returns bare URI for the request, NOT including query parameters.
    // fn get_bare_uri() -> Result<http::Uri, InvalidUri> {
    //     http::Uri::from_str(&format!(
    //         "{}{}?",
    //         crate::TWITCH_HELIX_URL,
    //         <Self as Request>::PATH,
    //     ))
    //     .map_err(Into::into)
    // }
}

/// Helix endpoint GETs information
pub trait RequestGet: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        // let uri = self.get_uri()?;

        http::Request::builder()
            .method(http::Method::GET)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .body(Vec::with_capacity(0))
            .map_err(Into::into)
    }

    /// Parse response. Override for different behavior
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestGetError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            HelixRequestGetError::Utf8Error(
                response.body().clone(),
                e,
                uri.clone(),
            )
        })?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<HelixRequestError>(&text)
        {
            return Err(HelixRequestGetError::Error {
                error,
                status: status
                    .try_into()
                    .unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        let response: InnerResponse<_> =
            serde_json::from_str(&text).map_err(|e| {
                HelixRequestGetError::DeserializeError(
                    text.to_string(),
                    e,
                    uri.clone(),
                )
            })?;
        Ok(Response {
            data: response.data,
            request,
        })
    }
}

/// Deserialize 'null' as <T as Default>::Default
fn deserialize_default_from_empty_string<'de, D, T>(
    deserializer: D
) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned + Default,
{
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::String(string) if string.is_empty() => Ok(None),
        other => {
            Ok(serde_json::from_value(other)
                .map_err(serde::de::Error::custom)?)
        }
    }
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
pub enum HelixRequestGetError {
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
pub enum HelixRequestPutError {
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
pub enum HelixRequestPostError {
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
pub struct HelixRequestPatchError {
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
pub enum HelixRequestDeleteError {
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

// /// Helix endpoint POSTs information
// #[cfg_attr(nightly, doc(spotlight))]
// pub trait RequestPost: Request {
//     /// Body parameters
//     type Body: serde::Serialize;

//     /// Create body text from [`RequestPost::Body`]
//     fn body(
//         &self,
//         body: &Self::Body,
//     ) -> Result<String, BodyError> {
//         serde_json::to_string(body).map_err(Into::into)
//     }

//     /// Create a [`http::Request`] from this [`Request`] in your client
//     fn create_request(
//         &self,
//         body: Self::Body,
//         token: &str,
//         client_id: &str,
//     ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
//         let uri = self.get_uri()?;

//         let body = self.body(&body)?;
//         // eprintln!("\n\nbody is ------------ {} ------------", body);

//         let mut bearer =
//             http::HeaderValue::from_str(&format!("Bearer {}",
// token)).map_err(                 |_| {
//                     CreateRequestError::Custom(
//                         "Could not make token into headervalue".into(),
//                     )
//                 },
//             )?;
//         bearer.set_sensitive(true);
//         http::Request::builder()
//             .method(http::Method::POST)
//             .uri(uri)
//             .header("Client-ID", client_id)
//             .header("Content-Type", "application/json")
//             .header(http::header::AUTHORIZATION, bearer)
//             .body(body.into_bytes())
//             .map_err(Into::into)
//     }

//     /// Parse response. Override for different behavior
//     fn parse_response(
//         request: Option<Self>,
//         uri: &http::Uri,
//         response: http::Response<Vec<u8>>,
//     ) -> Result<
//         Response<Self, <Self as Request>::Response>,
//         HelixRequestPostError,
//     >
//     where
//         Self: Sized,
//     {
//         let text = std::str::from_utf8(&response.body()).map_err(|e| {
//             HelixRequestPostError::Utf8Error(
//                 response.body().clone(),
//                 e,
//                 uri.clone(),
//             )
//         })?;
//         if let Ok(HelixRequestError {
//             error,
//             status,
//             message,
//         }) = serde_json::from_str::<HelixRequestError>(&text)
//         {
//             return Err(HelixRequestPostError::Error {
//                 error,
//                 status: status
//                     .try_into()
//                     .unwrap_or(http::StatusCode::BAD_REQUEST),
//                 message,
//                 uri: uri.clone(),
//                 body: response.body().clone(),
//             });
//         }
//         let response: InnerResponse<<Self as Request>::Response> =
//             serde_json::from_str(&text).map_err(|e| {
//                 HelixRequestPostError::DeserializeError(
//                     text.to_string(),
//                     e,
//                     uri.clone(),
//                 )
//             })?;
//         Ok(Response {
//             data: response.data,
//             pagination: response.pagination.cursor,
//             request,
//         })
//     }
// }

// /// Helix endpoint PATCHs information
// #[cfg_attr(nightly, doc(spotlight))]
// pub trait RequestPatch: Request
// where <Self as Request>::Response: std::convert::TryFrom<
//         http::StatusCode,
//         Error = std::borrow::Cow<'static, str>,
//     >
// {
//     /// Body parameters
//     type Body: serde::Serialize;

//     /// Create body text from [`RequestPatch::Body`]
//     fn body(
//         &self,
//         body: &Self::Body,
//     ) -> Result<String, BodyError> {
//         serde_json::to_string(body).map_err(Into::into)
//     }

//     /// Create a [`http::Request`] from this [`Request`] in your client
//     fn create_request(
//         &self,
//         body: Self::Body,
//         token: &str,
//         client_id: &str,
//     ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
//         let uri = self.get_uri()?;

//         let body = self.body(&body)?;
//         // eprintln!("\n\nbody is ------------ {} ------------", body);

//         let mut bearer =
//             http::HeaderValue::from_str(&format!("Bearer {}",
// token)).map_err(                 |_| {
//                     CreateRequestError::Custom(
//                         "Could not make token into headervalue".into(),
//                     )
//                 },
//             )?;
//         bearer.set_sensitive(true);
//         http::Request::builder()
//             .method(http::Method::PATCH)
//             .uri(uri)
//             .header("Client-ID", client_id)
//             .header("Content-Type", "application/json")
//             .header(http::header::AUTHORIZATION, bearer)
//             .body(body.into_bytes())
//             .map_err(Into::into)
//     }

//     /// Parse response. Override for different behavior
//     fn parse_response(
//         uri: &http::Uri,
//         response: http::Response<Vec<u8>>,
//     ) -> Result<<Self as Request>::Response, HelixRequestPatchError>
//     where
//         Self: Sized,
//     {
//         match response.status().try_into() {
//             Ok(result) => Ok(result),
//             Err(err) => Err(HelixRequestPatchError {
//                 status: response.status(),
//                 message: err.to_string(),
//                 uri: uri.clone(),
//                 body: response.body().clone(),
//             }),
//         }
//     }
// }

// /// Helix endpoint DELETEs information
// #[cfg_attr(nightly, doc(spotlight))]
// pub trait RequestDelete: Request {
//     /// Create a [`http::Request`] from this [`Request`] in your client
//     fn create_request(
//         &self,
//         token: &str,
//         client_id: &str,
//     ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
//         let uri = self.get_uri()?;

//         let mut bearer =
//             http::HeaderValue::from_str(&format!("Bearer {}",
// token)).map_err(                 |_| {
//                     CreateRequestError::Custom(
//                         "Could not make token into headervalue".into(),
//                     )
//                 },
//             )?;
//         bearer.set_sensitive(true);
//         http::Request::builder()
//             .method(http::Method::DELETE)
//             .uri(uri)
//             .header("Client-ID", client_id)
//             .header("Content-Type", "application/json")
//             .header(http::header::AUTHORIZATION, bearer)
//             .body(Vec::with_capacity(0))
//             .map_err(Into::into)
//     }

//     /// Parse response. Override for different behavior
//     fn parse_response(
//         uri: &http::Uri,
//         response: http::Response<Vec<u8>>,
//     ) -> Result<<Self as Request>::Response, HelixRequestDeleteError>
//     where
//         <Self as Request>::Response: std::convert::TryFrom<
//             http::StatusCode,
//             Error = std::borrow::Cow<'static, str>,
//         >,
//         Self: Sized,
//     {
//         let text = std::str::from_utf8(&response.body()).map_err(|e| {
//             HelixRequestDeleteError::Utf8Error(
//                 response.body().clone(),
//                 e,
//                 uri.clone(),
//             )
//         })?;
//         // eprintln!("\n\nmessage is ------------ {} ------------", text);

//         if let Ok(HelixRequestError {
//             error,
//             status,
//             message,
//         }) = serde_json::from_str::<HelixRequestError>(&text)
//         {
//             return Err(HelixRequestDeleteError::Error {
//                 error,
//                 status: status
//                     .try_into()
//                     .unwrap_or(http::StatusCode::BAD_REQUEST),
//                 message,
//                 uri: uri.clone(),
//             });
//         }

//         match response.status().try_into() {
//             Ok(result) => Ok(result),
//             Err(err) => Err(HelixRequestDeleteError::Error {
//                 error: String::new(),
//                 status: response.status(),
//                 message: err.to_string(),
//                 uri: uri.clone(),
//             }),
//         }
//     }
// }

// /// Helix endpoint PUTs information
// #[cfg_attr(nightly, doc(spotlight))]
// pub trait RequestPut: Request {
//     /// Create a [`http::Request`] from this [`Request`] in your client
//     fn create_request(
//         &self,
//         token: &str,
//         client_id: &str,
//     ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
//         let uri = self.get_uri()?;

//         let mut bearer =
//             http::HeaderValue::from_str(&format!("Bearer {}",
// token)).map_err(                 |_| {
//                     CreateRequestError::Custom(
//                         "Could not make token into headervalue".into(),
//                     )
//                 },
//             )?;
//         bearer.set_sensitive(true);
//         http::Request::builder()
//             .method(http::Method::PUT)
//             .uri(uri)
//             .header("Client-ID", client_id)
//             .header("Content-Type", "application/json")
//             .header(http::header::AUTHORIZATION, bearer)
//             .body(Vec::with_capacity(0))
//             .map_err(Into::into)
//     }

//     /// Parse response. Override for different behavior
//     fn parse_response(
//         uri: &http::Uri,
//         response: http::Response<Vec<u8>>,
//     ) -> Result<<Self as Request>::Response, HelixRequestPutError>
//     where
//         <Self as Request>::Response: std::convert::TryFrom<
//             http::StatusCode,
//             Error = std::borrow::Cow<'static, str>,
//         >,
//         Self: Sized,
//     {
//         let text = std::str::from_utf8(&response.body()).map_err(|e| {
//             HelixRequestPutError::Utf8Error(
//                 response.body().clone(),
//                 e,
//                 uri.clone(),
//             )
//         })?;
//         // eprintln!("\n\nmessage is ------------ {} ------------", text);

//         if let Ok(HelixRequestError {
//             error,
//             status,
//             message,
//         }) = serde_json::from_str::<HelixRequestError>(&text)
//         {
//             return Err(HelixRequestPutError::Error {
//                 error,
//                 status: status
//                     .try_into()
//                     .unwrap_or(http::StatusCode::BAD_REQUEST),
//                 message,
//                 uri: uri.clone(),
//             });
//         }

//         match response.status().try_into() {
//             Ok(result) => Ok(result),
//             Err(err) => Err(HelixRequestPutError::Error {
//                 error: String::new(),
//                 status: response.status(),
//                 message: err.to_string(),
//                 uri: uri.clone(),
//             }),
//         }
//     }
// }
