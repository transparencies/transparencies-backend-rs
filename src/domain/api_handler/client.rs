//! Implementation side of the core http
//! client logic of the application

use api_client::{
    client::Client,
    error::ClientRequestError,
    request::{
        Request,
        RequestGet,
    },
    response::Response,
};
use derive_getters::Getters;
use http::StatusCode;
use url::Url;

use crate::{
    domain::types::{
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
    APP_USER_AGENT,
    CLIENT_CONNECTION_TIMEOUT,
    CLIENT_REQUEST_TIMEOUT,
};

/// Datastructure storing our different [`ApiClient`]s
#[derive(Getters, Debug, Clone)]
pub struct ApiClient<'a, C>
    where C: Client<'a>,
{
    /// Client for aoe2net requests
    pub aoe2net: A2NClient<'a, C>,
}

#[derive(Clone, Debug)]
pub struct A2NClient<'a, C>
    where C: Client<'a>,
{
    client: C,
    _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
}

impl<'a, C> Default for A2NClient<'a, C> where C: Client<'a> + Default,
{
    fn default() -> A2NClient<'a, C> {
        A2NClient::new()
    }
}

impl<C: Client<'static>> ApiClient<'static, C> {
    /// Create a new [`ApiClient`]
    #[must_use]
    pub fn new() -> ApiClient<'static, C>
        where C: Clone + Default, {
        ApiClient { aoe2net: A2NClient::new() }
    }
}

impl<C: Client<'static>> Default for ApiClient<'static, C>
    where C: Clone + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, C: Client<'a>> A2NClient<'a, C> {
    /// Create a new client with an existing client
    #[must_use]
    pub fn with_client(client: C) -> A2NClient<'a, C> {
        A2NClient { client,
                    _pd: std::marker::PhantomData::default() }
    }

    /// Create a new [`HelixClient`] with a default
    /// [`HttpClient`][`crate::HttpClient`]
    #[must_use]
    pub fn new() -> A2NClient<'a, C>
        where C: Default + Client<'a>, {
        let client = C::default();
        A2NClient::with_client(client)
    }

    /// Retrieve a clone of the [`HttpClient`][`crate::HttpClient`] inside this
    /// [`HelixClient`]
    pub fn clone_client(&self) -> C
        where C: Clone, {
        self.client.clone()
    }

    /// Request on a valid [`RequestGet`] endpoint
    ///
    ///  # Errors
    // TODO
    pub async fn req_get<R, D>(
        &'a self,
        request: R)
        -> Result<Response<R, D>, ClientRequestError<<C as Client<'a>>::Error>>
        where R: Request<Response = D> + Request + RequestGet,
              D: serde::de::DeserializeOwned + PartialEq,
    {
        let req = request.create_request()?;
        let uri = req.uri().clone();
        let response = self.client
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
    //     T: Token + ?Sized,
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
    //     T: Token + ?Sized,
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
    //     T: Token + ?Sized,
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
    //     T: Token + ?Sized,
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

impl Default for FileFormat {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Default for File {
    fn default() -> Self {
        File::builder().name(String::new())
                       .ext(FileFormat::default())
                       .build()
    }
}

impl std::fmt::Display for File {
    fn fmt(&self,
           f: &mut std::fmt::Formatter<'_>)
           -> std::fmt::Result {
        write!(f, "{}.{}", self.name(), self.ext().as_ref().to_lowercase())
    }
}

impl File {
    #[must_use]
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

impl Default for GithubFileRequest {
    fn default() -> Self {
        GithubFileRequest::builder()
            .client(
                reqwest::Client::builder()
                    .user_agent(*APP_USER_AGENT)
                    .timeout(*CLIENT_REQUEST_TIMEOUT)
                    .connect_timeout(*CLIENT_CONNECTION_TIMEOUT)
                    .use_rustls_tls()
                    .https_only(true)
                    .build()
                    .unwrap(),
            )
            .url(Url::parse("https://raw.githubusercontent.com").unwrap())
            .build()
    }
}

impl GithubFileRequest {
    /// Executes the Request
    ///
    /// # Errors
    ///
    /// see [`reqwest::Error`]
    #[inline]
    pub async fn execute(&self) -> Result<reqwest::Response, FileRequestError> {
        Ok(self.client().get(self.url().clone()).send().await?)
    }
}

impl Default for ApiRequest {
    fn default() -> Self {
        ApiRequest::builder().client(reqwest::Client::default())
                             .root(Url::parse("https://aoe2.net/api").unwrap())
                             .endpoint(String::new())
                             .query(Vec::new())
                             .build()
    }
}

impl ApiRequest {
    /// Executes the Request
    ///
    /// # Errors
    ///
    /// see [`reqwest::Error`]
    pub async fn execute<R>(&self) -> Result<R, ApiRequestError>
        where R: for<'de> serde::Deserialize<'de>, {
        let response =
            self.client()
                .get(&format!("{}/{}", &self.root().as_str(), &self.endpoint()))
                .query(&self.query())
                .send()
                .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::NOT_FOUND => {
                return Err(ApiRequestError::NotFoundResponse {
                    root: self.root().as_str().to_string(),
                    endpoint: self.endpoint().to_string(),
                    query: self.query().clone(),
                })
            }
            _ => {
                return Err(ApiRequestError::HttpClientErrorWithStatusCode(
                    response.status(),
                ))
            }
        }
    }
}
