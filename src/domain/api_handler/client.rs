//! Core client logic of the application

use log::{debug, error, info, trace, warn};
use reqwest::Request;
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};

use ::serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use crate::domain::api_handler::response::{
    aoc_ref::{platforms, players, teams},
    aoe2net::last_match::PlayerLastMatch,
};

use std::fmt;

use strum::AsRefStr;

/// App-Name as USERAGENT
pub(crate) static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Time-outs for http-clients
pub(crate) static CLIENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
pub(crate) static CLIENT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, AsRefStr)]
pub enum FileFormat {
    Toml,
    Json,
    Yaml,
    Ron,
    Xml,
    Url,
    Uninitialized,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response<T> {
    pub response: T,
}

impl Default for FileFormat {
    fn default() -> Self {
        Self::Uninitialized
    }
}

// impl std::fmt::Display for FileFormat {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         write!(f, "{}", self.to_string().to_lowercase())
//     }
// }

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub ext: FileFormat,
}

impl Default for File {
    fn default() -> Self {
        Self {
            name: String::new(),
            ext: FileFormat::default(),
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.ext.as_ref().to_lowercase())
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub aoe2net: reqwest::Client,
    pub github: reqwest::Client,
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

// root: https://raw.githubusercontent.com
// user: SiegeEngineers
// repo: aoc-reference-data
// uri: master/data
// file: File {
//         name: players
//         ext: FileFormat::Yaml
// }
// https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/players.yaml
// &format!("{}/{}/{}/{}/{}", &self.root, &self.user, &self.repo, &self.uri,
// &self.file)
#[derive(Builder, Debug)]
#[builder(public, setter(into))]
pub struct GithubFileRequest {
    // #[builder(setter(skip))]
    client: reqwest::Client,
    root: String,
    user: String,
    repo: String,
    uri: String,
    file: File,
}

impl Default for GithubFileRequest {
    fn default() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(CLIENT_REQUEST_TIMEOUT)
                .connect_timeout(CLIENT_CONNECTION_TIMEOUT)
                .use_rustls_tls()
                .https_only(true)
                .build()
                .unwrap(),
            root: String::new(),
            user: String::new(),
            repo: String::new(),
            uri: String::new(),
            file: File::default(),
        }
    }
}

impl GithubFileRequest {
    pub async fn execute(&self) -> Result<reqwest::Response> {
        Ok(self
            .client
            .get(&format!(
                "{}/{}/{}/{}/{}",
                &self.root, &self.user, &self.repo, &self.uri, &self.file
            ))
            .send()
            .await?)
    }
}

#[derive(Builder, Debug)]
#[builder(public, setter(into))]
pub struct ApiRequest {
    // #[builder(setter(skip))]
    client: reqwest::Client,
    root: String,
    endpoint: String,
    query: Vec<(String, String)>,
}

impl Default for ApiRequest {
    fn default() -> Self {
        Self {
            client: reqwest::Client::default(),
            root: String::new(),
            endpoint: String::new(),
            query: Vec::new(),
        }
    }
}

impl ApiRequest {
    pub async fn execute<R>(&self) -> Result<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        Ok(self
            .client
            .get(&format!("{}/{}", &self.root, &self.endpoint))
            .query(&self.query)
            .send()
            .await?
            .json::<R>()
            .await?)
    }
}
