//! Implementation side of the core http
//! client logic of the application

use crate::domain::types::{
    error::{
        ApiRequestError,
        FileRequestError,
    },
    requests::{
        ApiClient,
        ApiRequest,
        File,
        FileFormat,
        GithubFileRequest,
    },
};
use http::StatusCode;
use std::time::Duration;
use url::Url;

/// Our app name as USERAGENT for the clients
pub(crate) static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Timeout for http-client requests
pub(crate) static CLIENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
/// Timeout for http-connections
pub(crate) static CLIENT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

impl Default for FileFormat {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl Default for File {
    fn default() -> Self {
        File::builder()
            .name(String::new())
            .ext(FileFormat::default())
            .build()
    }
}

impl std::fmt::Display for File {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}.{}", self.name(), self.ext().as_ref().to_lowercase())
    }
}

impl File {
    #[must_use]
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self {
            aoe2net: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(CLIENT_REQUEST_TIMEOUT)
                .connect_timeout(Duration::from_secs(60))
                .use_rustls_tls()
                .https_only(true)
                .build()
                .unwrap(),
            github: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(CLIENT_REQUEST_TIMEOUT)
                .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
                .use_rustls_tls()
                .https_only(true)
                .build()
                .unwrap(),
        }
    }
}

impl ApiClient {
    /// Builds a new [`ApiClient`] with setting HTTPs to enabled/disabled
    ///
    /// # Arguments
    /// * `enabled` - `True` = HTTPs enabled, `False` = HTTPs disabled
    ///
    /// # Panics
    /// This function panics if [`reqwest::Client`] can not be build.
    #[inline]
    #[must_use]
    pub fn new_with_https(enabled: bool) -> Self {
        Self {
            aoe2net: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(CLIENT_REQUEST_TIMEOUT)
                .connect_timeout(Duration::from_secs(60))
                .use_rustls_tls()
                .https_only(enabled)
                .build()
                .unwrap(),
            github: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(CLIENT_REQUEST_TIMEOUT)
                .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
                .use_rustls_tls()
                .https_only(enabled)
                .build()
                .unwrap(),
        }
    }
}

impl Default for GithubFileRequest {
    fn default() -> Self {
        GithubFileRequest::builder()
            .client(
                reqwest::Client::builder()
                    .user_agent(APP_USER_AGENT)
                    .timeout(CLIENT_REQUEST_TIMEOUT)
                    .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
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
        Ok(self.client().get(self.url().to_owned()).send().await?)
    }
}

impl Default for ApiRequest {
    fn default() -> Self {
        ApiRequest::builder()
            .client(reqwest::Client::default())
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
    where R: for<'de> serde::Deserialize<'de> {
        let response = self
            .client()
            .get(&format!("{}/{}", &self.root().as_str(), &self.endpoint()))
            .query(&self.query())
            .send()
            .await;

        match response {
            Err(err) => {
                if let Some(status_code) = err.status() {
                    match status_code {
                        StatusCode::NOT_FOUND => {
                            return Err(ApiRequestError::NotFoundResponse {
                                root: self.root().as_str().to_string(),
                                endpoint: self.endpoint().to_string(),
                                query: self.query().to_vec(),
                            })
                        }
                        _ => {
                            return Err(
                                ApiRequestError::HttpClientErrorWithStatusCode(
                                    status_code,
                                ),
                            )
                        }
                    }
                }
                else {
                    return Err(ApiRequestError::HttpClientError(err));
                }
            }
            Ok(response) => Ok(response.json().await?),
        }
    }
}
