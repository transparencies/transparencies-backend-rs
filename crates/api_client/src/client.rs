//! Implementation side of the core http
//! client logic of the application

use std::{
    error::Error,
    future::Future,
};

use reqwest::Client as ReqwestClient;

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
    fn req(&'a self,
           request: Req)
           -> BoxedFuture<'a, Result<Response, <Self as Client>::Error>>;
}

impl<'a> Client<'a> for ReqwestClient {
    type Error = reqwest::Error;

    fn req(&'a self,
           request: Req)
           -> BoxedFuture<'static, Result<Response, Self::Error>> {
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

#[derive(Debug, Default, thiserror::Error, Clone)]
/// A client that will never work, used to trick documentation tests
#[error("this client does not do anything, only used for documentation test that only checks")]
pub struct DummyHttpClient;

impl<'a> Client<'a> for DummyHttpClient {
    type Error = DummyHttpClient;

    fn req(&'a self,
           _: Req)
           -> BoxedFuture<'a, Result<Response, Self::Error>> {
        Box::pin(async { Err(DummyHttpClient) })
    }
}
