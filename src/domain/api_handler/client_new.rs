//! Implementation side of the core http
//! client logic of the application

use std::time::Duration;

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

/// Our app name as USERAGENT for the clients
pub(crate) static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Timeout for http-client requests
pub(crate) static CLIENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
/// Timeout for http-connections
pub(crate) static CLIENT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

/// Datastructure storing our different [`ApiClient`]s
#[derive(Getters, Debug, Clone)]
pub struct ApiClient<'a, C>
where C: Client<'a>
{
    /// Client for aoe2net requests
    pub aoe2net: A2NClient<'a, C>,
}

#[derive(Clone, Debug)]
pub struct A2NClient<'a, C>
where C: Client<'a>
{
    client: C,
    _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
}

impl<'a, C> Default for A2NClient<'a, C>
where C: Client<'a> + Default
{
    fn default() -> A2NClient<'a, C> {
        A2NClient::new()
    }
}

impl<C: Client<'static>> ApiClient<'static, C> {
    /// Create a new [`ApiClient`]
    #[must_use]
    pub fn new() -> ApiClient<'static, C>
    where C: Clone + Default {
        ApiClient {
            aoe2net: A2NClient::new(),
        }
    }
}

impl<'a, C: Client<'a>> A2NClient<'a, C> {
    /// Create a new client with an existing client
    #[must_use]
    pub fn with_client(client: C) -> A2NClient<'a, C> {
        A2NClient {
            client,
            _pd: std::marker::PhantomData::default(),
        }
    }

    /// Create a new [`HelixClient`] with a default
    /// [`HttpClient`][`crate::HttpClient`]
    #[must_use]
    pub fn new() -> A2NClient<'a, C>
    where C: Default + Client<'a> {
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
    ///  # Errors
    // TODO
    pub async fn req_get<R, D>(
        &'a self,
        request: R,
    ) -> Result<Response<R, D>, ClientRequestError<<C as Client<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestGet,
        D: serde::de::DeserializeOwned + PartialEq,
    {
        let req = request.create_request()?;
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
