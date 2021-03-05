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
    pub aoe2net: reqwest::Client,
    pub github: reqwest::Client,
}

/// `File` datastructure to mimmick a file for Github requests
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct File {
    name: String,
    ext: FileFormat,
}

/// `FileFormat` stores all the parsable files that we can pull in
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
    pub response: T,
}
/// Datastructure to deal with `GET` API-Requests
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct ApiRequest {
    client: reqwest::Client,
    root: String,
    endpoint: String,
    query: Vec<(String, String)>,
}

/// Datastructure to deal with `FileRequests` towards Github
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct GithubFileRequest {
    client: reqwest::Client,
    root: String,
    user: String,
    repo: String,
    uri: String,
    file: File,
}
