use derive_getters::{Dissolve, Getters};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use typed_builder::TypedBuilder;

#[derive(Getters, Debug, Clone)]
pub struct ApiClient {
    pub aoe2net: reqwest::Client,
    pub github: reqwest::Client,
}

#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct File {
    name: String,
    ext: FileFormat,
}

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
#[derive(Getters, TypedBuilder, Debug, Clone)]
#[builder(field_defaults(setter(into)))]
pub struct ApiRequest {
    client: reqwest::Client,
    root: String,
    endpoint: String,
    query: Vec<(String, String)>,
}

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
