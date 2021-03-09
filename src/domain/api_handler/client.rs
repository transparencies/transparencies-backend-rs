//! Implementation side of the core http
//! client logic of the application

use crate::domain::types::{
    error::FileRequestError,
    requests::{
        ApiClient,
        ApiRequest,
        File,
        FileFormat,
        GithubFileRequest,
    },
};
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
            .root(Url::parse("https://raw.githubusercontent.com").unwrap())
            .user(String::new())
            .repo(String::new())
            .uri(String::new())
            .file(File::default())
            .build()
    }
}

impl GithubFileRequest {
    /// Executes the Request
    ///
    /// # Errors
    ///
    /// see [`reqwest::Error`]
    pub async fn execute(&self) -> Result<reqwest::Response, FileRequestError> {
        Ok(self
            .client()
            .get(
                self.root()
                    .join(self.user())?
                    .join(self.repo())?
                    .join(self.uri())?
                    .join(&(self.file().display()))?,
            )
            .send()
            .await?)
    }

    // &format!(
    //                 "{}/{}/{}/{}/{}",
    //                 &self.root(),
    //                 &self.user(),
    //                 &self.repo(),
    //                 &self.uri(),
    //                 &self.file()
    //             )
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
    pub async fn execute<R>(&self) -> Result<R, reqwest::Error>
    where R: for<'de> serde::Deserialize<'de> {
        Ok(self
            .client()
            .get(&format!("{}/{}", &self.root(), &self.endpoint()))
            .query(&self.query())
            .send()
            .await?
            .json()
            .await?)
    }
}
