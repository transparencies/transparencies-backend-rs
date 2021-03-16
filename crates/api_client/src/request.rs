use std::{
    convert::TryInto,
    str::FromStr,
};

use serde::Deserialize;

use crate::{
    response::{
        InnerResponse,
        Response,
    },
    ser,
};

pub use crate::error::*;

/// A request is an API endpoint
#[async_trait::async_trait]
pub trait Request: serde::Serialize {
    /// The path to the endpoint relative to the helix root. eg. `channels` for [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    const ROOT: &'static str;
    /// The path to the endpoint relative to the helix root. eg. `channels` for [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    const PATH: &'static str;
    /// Response type. twitch's response will  deserialize to this.
    type Response: serde::de::DeserializeOwned + PartialEq;
    /// Defines layout of the url parameters.
    fn query(&self) -> Result<String, ser::Error> {
        ser::to_string(&self)
    }
    /// Returns full URI for the request, including query parameters.
    fn get_uri(&self) -> Result<http::Uri, InvalidUri> {
        http::Uri::from_str(&format!(
            "{}{}?{}",
            <Self as Request>::ROOT,
            <Self as Request>::PATH,
            self.query()?
        ))
        .map_err(Into::into)
    }
    /// Returns bare URI for the request, NOT including query parameters.
    fn get_bare_uri() -> Result<http::Uri, InvalidUri> {
        http::Uri::from_str(&format!(
            "{}{}?",
            <Self as Request>::ROOT,
            <Self as Request>::PATH,
        ))
        .map_err(Into::into)
    }
}

/// Helix endpoint GETs information
pub trait RequestGet: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        http::Request::builder()
            .method(http::Method::GET)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .body(Vec::with_capacity(0))
            .map_err(Into::into)
    }

    /// Create a [`http::Request`] with bearer auth from this [`Request`] in
    /// your client
    fn create_request_with_bearer(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(
                |_| {
                    CreateRequestError::Custom(
                        "Could not make token into headervalue".into(),
                    )
                },
            )?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::GET)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0))
            .map_err(Into::into)
    }

    /// Parse response. Override for different behavior
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, ApiRequestGetError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            ApiRequestGetError::Utf8Error(
                response.body().clone(),
                e,
                uri.clone(),
            )
        })?;
        // eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(ApiRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<ApiRequestError>(&text)
        {
            return Err(ApiRequestGetError::Error {
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
                ApiRequestGetError::DeserializeError(
                    text.to_string(),
                    e,
                    uri.clone(),
                )
            })?;
        Ok(Response {
            data: response.data,
            pagination: None,
            // TODO: pagination: response.pagination.cursor,
            request,
        })
    }
}

/// Deserialize 'null' as <T as Default>::Default
// TODO: For now allow it
#[allow(dead_code)]
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
