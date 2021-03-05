#![allow(clippy::used_underscore_binding)]
#![allow(clippy::empty_enum)]
//! Datatypes used to create and execute requests

use derive_getters::Getters;
use serde::{
    Deserialize,
    Serialize,
};
use strum::AsRefStr;
use typed_builder::TypedBuilder;

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
    /// [Fileformat]
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
/// A generic response implementation
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response<T> {
    /// Contains the responses of requests
    pub response: T,
}
/// Datastructure to deal with `GET` API-Requests
#[allow(missing_docs)]
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct ApiRequest {
    /// A clone of a [reqwest::Client] for connection pooling
    client: reqwest::Client,
    /// The API root
    root: String,
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
    /// The root
    root: String,
    /// Username
    user: String,
    /// Repository name
    repo: String,
    /// The identifier where to find the top-folder of the file in the
    /// repository
    uri: String,
    /// The File itself
    file: File,
}
