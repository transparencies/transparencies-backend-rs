//! Implementation side of the core http
//! client logic of the application

use crate::{
    domain::{
        api_handler::client_new::request::{
            Request,
            RequestGet,
        },
        types::{
            error::{
                ApiRequestError,
                FileRequestError,
            },
            requests::{
                ApiRequest,
                File,
                FileFormat,
                GithubFileRequest,
            },
        },
    },
    request::{
        Request,
        RequestGet,
    },
};
use derive_getters::Getters;

use http::StatusCode;
use reqwest::Client as ReqwestClient;
use std::{
    error::Error,
    future::Future,
    time::Duration,
};
use url::Url;

/// Our app name as USERAGENT for the clients
pub(crate) static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Timeout for http-client requests
pub(crate) static CLIENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
/// Timeout for http-connections
pub(crate) static CLIENT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

/// A boxed future, mimics `futures::future::BoxFuture`
pub type BoxedFuture<'a, T> =
    std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The request type we're expecting with body.
pub type Req = http::Request<Vec<u8>>;
/// The response type we're expecting with body
pub type Response = http::Response<Vec<u8>>;

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

/// Datastructure storing different our `ApiClients`
#[derive(Getters, Debug, Clone)]
pub struct ApiClient<'a, C>
where C: Client<'a>
{
    /// Client for aoe2net requests
    pub aoe2net: A2NClient<'a, C>,
    /// Client for github requests
    pub github: GitClient<'a, C>,
}

#[derive(Clone)]
pub struct A2NClient<'a, C>
where C: Client<'a>
{
    client: C,
    _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
}

#[derive(Clone)]
pub struct GitClient<'a, C>
where C: Client<'a>
{
    client: C,
    _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
}

impl<C: Client<'static>> ApiClient<'static, C> {
    /// Create a new [`ApiClient`]
    pub fn new() -> ApiClient<'static, C>
    where C: Clone + Default {
        ApiClient {
            aoe2net: A2NClient::new(),
            github: GitClient::new(),
        }
    }
}

impl<'a, C: Client<'a>> A2NClient<'a, C> {
    /// Create a new client with an existing client
    pub fn with_client(client: C) -> A2NClient<'a, C> {
        A2NClient {
            client,
            _pd: std::marker::PhantomData::default(),
        }
    }

    /// Create a new [`HelixClient`] with a default
    /// [`HttpClient`][crate::HttpClient]
    pub fn new() -> A2NClient<'a, C>
    where C: Default {
        let client = C::default();
        A2NClient::with_client(client)
    }

    /// Retrieve a clone of the [`HttpClient`][crate::HttpClient] inside this
    /// [`HelixClient`]
    pub fn clone_client(&self) -> C
    where C: Clone {
        self.client.clone()
    }

    /// Request on a valid [`RequestGet`] endpoint
    ///
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// #   use twitch_api2::helix::{HelixClient, channels};
    /// #   let token = Box::new(twitch_oauth2::UserToken::from_existing_unchecked(
    /// #       twitch_oauth2::AccessToken::new("totallyvalidtoken".to_string()), None,
    /// #       twitch_oauth2::ClientId::new("validclientid".to_string()), None, "justintv".to_string(), "1337".to_string(), None, None));
    ///     let req = channels::GetChannelInformationRequest::builder().broadcaster_id("123456").build();
    ///     let client = HelixClient::new();
    /// # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
    ///
    ///     let response = client.req_get(req, &token).await;
    /// # }
    /// # // fn main() {run()}
    /// ```
    pub async fn req_get<R, D, T>(
        &'a self,
        request: R,
    ) -> Result<Response<R, D>, ClientRequestError<<C as Client<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestGet,
        D: serde::de::DeserializeOwned + PartialEq,
    {
        let req = request.create_request(
            token.token().secret(),
            token.client_id().as_str(),
        )?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }

    // /// Request on a valid [`RequestPost`] endpoint
    // pub async fn req_post<R, B, D, T>(
    //     &'a self,
    //     request: R,
    //     body: B,
    //     token: &T,
    // ) -> Result<
    //     Response<R, D>,
    //     ClientRequestError<<C as crate::HttpClient<'a>>::Error>,
    // >
    // where
    //     R: Request<Response = D> + Request + RequestPost<Body = B>,
    //     B: serde::Serialize,
    //     D: serde::de::DeserializeOwned + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = request.create_request(
    //         body,
    //         token.token().secret(),
    //         token.client_id().as_str(),
    //     )?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?;
    //     <R>::parse_response(Some(request), &uri,
    // response).map_err(Into::into) }

    // /// Request on a valid [`RequestPatch`] endpoint
    // pub async fn req_patch<R, B, D, T>(
    //     &'a self,
    //     request: R,
    //     body: B,
    //     token: &T,
    // ) -> Result<D, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    // where
    //     R: Request<Response = D> + Request + RequestPatch<Body = B>,
    //     B: serde::Serialize,
    //     D: std::convert::TryFrom<
    //             http::StatusCode,
    //             Error = std::borrow::Cow<'static, str>,
    //         > + serde::de::DeserializeOwned
    //         + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = request.create_request(
    //         body,
    //         token.token().secret(),
    //         token.client_id().as_str(),
    //     )?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?;
    //     <R>::parse_response(&uri, response).map_err(Into::into)
    // }

    // /// Request on a valid [`RequestDelete`] endpoint
    // pub async fn req_delete<R, D, T>(
    //     &'a self,
    //     request: R,
    //     token: &T,
    // ) -> Result<D, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    // where
    //     R: Request<Response = D> + Request + RequestDelete,
    //     D: std::convert::TryFrom<
    //             http::StatusCode,
    //             Error = std::borrow::Cow<'static, str>,
    //         > + serde::de::DeserializeOwned
    //         + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = request.create_request(
    //         token.token().secret(),
    //         token.client_id().as_str(),
    //     )?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?;
    //     <R>::parse_response(&uri, response).map_err(Into::into)
    // }

    // /// Request on a valid [`RequestPut`] endpoint
    // pub async fn req_put<R, D, T>(
    //     &'a self,
    //     request: R,
    //     token: &T,
    // ) -> Result<D, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    // where
    //     R: Request<Response = D> + Request + RequestPut,
    //     D: std::convert::TryFrom<
    //             http::StatusCode,
    //             Error = std::borrow::Cow<'static, str>,
    //         > + serde::de::DeserializeOwned
    //         + PartialEq,
    //     T: TwitchToken + ?Sized,
    // {
    //     let req = request.create_request(
    //         token.token().secret(),
    //         token.client_id().as_str(),
    //     )?;
    //     let uri = req.uri().clone();
    //     let response = self
    //         .client
    //         .req(req)
    //         .await
    //         .map_err(ClientRequestError::RequestError)?;
    //     <R>::parse_response(&uri, response).map_err(Into::into)
    // }
}

impl<'a, C> Default for A2NClient<'a, C>
where C: Client<'a> + Default
{
    fn default() -> A2NClient<'a, C> {
        A2NClient::new()
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
    HelixRequestGetError(#[from] HelixRequestGetError),
    /// could not parse PUT response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestPutError(#[from] HelixRequestPutError),
    /// could not parse POST response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestPostError(#[from] HelixRequestPostError),
    /// could not parse PATCH response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestPatchError(#[from] HelixRequestPatchError),
    /// could not parse DELETE response
    // #[error(transparent)] // FIXME: https://github.com/yaahc/displaydoc/issues/15
    HelixRequestDeleteError(#[from] HelixRequestDeleteError),
    /// {0}
    Custom(std::borrow::Cow<'static, str>),
}
