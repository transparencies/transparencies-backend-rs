//! Implementation side of the core http
//! client logic of the application

use reqwest::Client as ReqwestClient;
use std::{
    error::Error,
    future::Future,
};

/// A boxed future, mimics `futures::future::BoxFuture`
pub type BoxedFuture<'a, T> =
    std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The request type we're expecting with body.
pub type Req = http::Request<Vec<u8>>;
/// The response type we're expecting with body
type Response = http::Response<Vec<u8>>;

/// A client that can do requests
pub trait Client<'a>: Send + 'a {
    /// Error returned by the client
    type Error: Error + Send + Sync + 'static;
    /// Send a request
    fn req(
        &'a self,
        request: Req,
    ) -> BoxedFuture<'a, Result<Response, <Self as Client>::Error>>;
}

impl<'a> Client<'a> for ReqwestClient {
    type Error = reqwest::Error;

    fn req(
        &'a self,
        request: Req,
    ) -> BoxedFuture<'static, Result<Response, Self::Error>> {
        // Reqwest plays really nice here and has a try_from on `http::Request`
        // -> `reqwest::Request`
        use std::convert::TryFrom;
        let req = match reqwest::Request::try_from(request) {
            Ok(req) => req,
            Err(e) => return Box::pin(async { Err(e) }),
        };
        // We need to "call" the execute outside the async closure to not
        // capture self.
        let fut = self.execute(req);
        Box::pin(async move {
            // Await the request and translate to `http::Response`
            let mut response = fut.await?;
            let mut result = http::Response::builder();
            let headers = result
                .headers_mut()
                // This should not fail, we just created the response.
                .expect("expected to get headers mut when building response");
            std::mem::swap(headers, response.headers_mut());
            let result = result.version(response.version());
            Ok(result
                .body(response.bytes().await?.as_ref().to_vec())
                .expect("mismatch reqwest -> http conversion should not fail"))
        })
    }
}

// ________________________________________________________________________________________
// ________________________________________________________________________________________
// ________________________________________________________________________________________
// ________________________________________________________________________________________

// impl Default for FileFormat {
//     fn default() -> Self {
//         FileFormat::Uninitialized
//     }
// }

// impl Default for File {
//     fn default() -> Self {
//         File::builder()
//             .name(String::new())
//             .ext(FileFormat::default())
//             .build()
//     }
// }

// impl std::fmt::Display for File {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         write!(f, "{}.{}", self.name(), self.ext().as_ref().to_lowercase())
//     }
// }

// impl File {
//     #[must_use]
//     pub fn display(&self) -> String {
//         format!("{}", self)
//     }
// }

// impl Default for ApiClient {
//     fn default() -> Self {
//         Self {
//             aoe2net: reqwest::Client::builder()
//                 .user_agent(APP_USER_AGENT)
//                 .timeout(CLIENT_REQUEST_TIMEOUT)
//                 .connect_timeout(Duration::from_secs(60))
//                 .use_rustls_tls()
//                 .https_only(true)
//                 .build()
//                 .unwrap(),
//             github: reqwest::Client::builder()
//                 .user_agent(APP_USER_AGENT)
//                 .timeout(CLIENT_REQUEST_TIMEOUT)
//                 .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
//                 .use_rustls_tls()
//                 .https_only(true)
//                 .build()
//                 .unwrap(),
//         }
//     }
// }

// impl ApiClient {
//     /// Builds a new [`ApiClient`] with setting HTTPs to enabled/disabled
//     ///
//     /// # Arguments
//     /// * `enabled` - `True` = HTTPs enabled, `False` = HTTPs disabled
//     ///
//     /// # Panics
//     /// This function panics if [`reqwest::Client`] can not be build.
//     #[inline]
//     #[must_use]
//     pub fn with_https(enabled: bool) -> Self {
//         Self {
//             aoe2net: reqwest::Client::builder()
//                 .user_agent(APP_USER_AGENT)
//                 .timeout(CLIENT_REQUEST_TIMEOUT)
//                 .connect_timeout(Duration::from_secs(60))
//                 .use_rustls_tls()
//                 .https_only(enabled)
//                 .build()
//                 .unwrap(),
//             github: reqwest::Client::builder()
//                 .user_agent(APP_USER_AGENT)
//                 .timeout(CLIENT_REQUEST_TIMEOUT)
//                 .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
//                 .use_rustls_tls()
//                 .https_only(enabled)
//                 .build()
//                 .unwrap(),
//         }
//     }
// }

// impl Default for GithubFileRequest {
//     fn default() -> Self {
//         GithubFileRequest::builder()
//             .client(
//                 reqwest::Client::builder()
//                     .user_agent(APP_USER_AGENT)
//                     .timeout(CLIENT_REQUEST_TIMEOUT)
//                     .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
//                     .use_rustls_tls()
//                     .https_only(true)
//                     .build()
//                     .unwrap(),
//             )
//             .url(Url::parse("https://raw.githubusercontent.com").unwrap())
//             .build()
//     }
// }

// impl GithubFileRequest {
//     /// Executes the Request
//     ///
//     /// # Errors
//     ///
//     /// see [`reqwest::Error`]
//     #[inline]
//     pub async fn execute(&self) -> Result<reqwest::Response,
// FileRequestError> {         Ok(self.client().get(self.url().to_owned()).
// send().await?)     }
// }

// impl Default for ApiRequest {
//     fn default() -> Self {
//         ApiRequest::builder()
//             .client(reqwest::Client::default())
//             .root(Url::parse("https://aoe2.net/api").unwrap())
//             .endpoint(String::new())
//             .query(Vec::new())
//             .build()
//     }
// }

// impl ApiRequest {
//     /// Executes the Request
//     ///
//     /// # Errors
//     ///
//     /// see [`reqwest::Error`]
//     pub async fn execute<R>(&self) -> Result<R, ApiRequestError>
//     where R: for<'de> serde::Deserialize<'de> {
//         let response = self
//             .client()
//             .get(&format!("{}/{}", &self.root().as_str(), &self.endpoint()))
//             .query(&self.query())
//             .send()
//             .await?;

//         match response.status() {
//             StatusCode::OK => Ok(response.json().await?),
//             StatusCode::NOT_FOUND => {
//                 return Err(ApiRequestError::NotFoundResponse {
//                     root: self.root().as_str().to_string(),
//                     endpoint: self.endpoint().to_string(),
//                     query: self.query().to_vec(),
//                 })
//             }
//             _ => {
//                 return Err(ApiRequestError::HttpClientErrorWithStatusCode(
//                     response.status(),
//                 ))
//             }
//         }
//     }
// }

#[derive(Debug, Default, thiserror::Error, Clone)]
/// A client that will never work, used to trick documentation tests
#[error("this client does not do anything, only used for documentation test that only checks")]
pub struct DummyHttpClient;

impl<'a> Client<'a> for DummyHttpClient {
    type Error = DummyHttpClient;

    fn req(
        &'a self,
        _: Req,
    ) -> BoxedFuture<'a, Result<Response, Self::Error>> {
        Box::pin(async { Err(DummyHttpClient) })
    }
}
