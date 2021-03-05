//! Implementation side of the core http
//! client logic of the application

use std::time::Duration;

use crate::domain::types::requests::{
    ApiClient,
    ApiRequest,
    File,
    FileFormat,
    GithubFileRequest,
};

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
            .root(String::new())
            .user(String::new())
            .repo(String::new())
            .uri(String::new())
            .file(File::default())
            .build()
    }
}

impl GithubFileRequest {
    pub async fn execute(&self) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .client()
            .get(&format!(
                "{}/{}/{}/{}/{}",
                &self.root(),
                &self.user(),
                &self.repo(),
                &self.uri(),
                &self.file()
            ))
            .send()
            .await?)
    }
}

impl Default for ApiRequest {
    fn default() -> Self {
        ApiRequest::builder()
            .client(reqwest::Client::default())
            .root(String::new())
            .endpoint(String::new())
            .query(Vec::new())
            .build()
    }
}

impl ApiRequest {
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
