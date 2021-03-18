#![allow(clippy::used_underscore_binding)]
#![allow(clippy::empty_enum)]
//! Datatypes used to create and execute requests

use derive_getters::Getters;
use strum::AsRefStr;
use typed_builder::TypedBuilder;
use url::Url;

/// Datastructure storing different our `ApiClients`
#[derive(Getters, Debug, Clone)]
pub struct ApiClient {
    /// Client for aoe2net requests
    pub aoe2net: reqwest::Client,
    /// Client for github requests
    pub github: reqwest::Client,
}

/// `File` datastructure to mimmick a file for Github requests
#[allow(missing_docs)]
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct File {
    /// Filename
    pub name: String,
    /// stores a [FileFormat]
    pub ext: FileFormat,
}

/// `FileFormat` stores all the parsable files that we can pull in
#[allow(missing_docs)]
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

impl FileFormat {
    /// Returns `true` if the `file_format` is [`Json`].
    #[must_use]
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json)
    }
}

/// Datastructure to deal with `GET` API-Requests
#[allow(missing_docs)]
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct ApiRequest {
    /// A clone of a [reqwest::Client] for connection pooling
    client: reqwest::Client,
    /// The API root
    root: Url,
    /// The Endpoint we are connecting to
    endpoint: String,
    /// A Vector of a tuple of query strings
    query: Vec<(String, String)>,
}

/// Datastructure to deal with `FileRequests` towards Github
#[allow(missing_docs)]
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct GithubFileRequest {
    /// A clone of a [reqwest::Client] for connection pooling
    client: reqwest::Client,
    /// The URL to execute the request for
    url: Url,
}
