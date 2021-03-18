//! Implementation side of the core http
//! client logic of the application

use crate::domain::types::{
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
};
use http::StatusCode;
use url::Url;

use crate::{
    APP_USER_AGENT,
    CLIENT_CONNECTION_TIMEOUT,
    CLIENT_REQUEST_TIMEOUT,
};

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
