//! Additional utility functions that are useful in different modules

use std::error::Error;

use crate::domain::types::{
    ApiRequest,
    File,
    GithubFileRequest,
};

/// Assembles a request for a file in a Github repository
pub(crate) fn build_github_request(
    git_client: reqwest::Client,
    root: &str,
    user: &str,
    repo: &str,
    uri: &str,
    file: &File,
) -> GithubFileRequest {
    GithubFileRequest::builder()
        .client(git_client)
        .root(root)
        .user(user)
        .repo(repo)
        .uri(uri)
        .file(file.clone())
        .build()
}

/// Assembles a `GET` request for an API
/// Refactoring: Use this function
pub(crate) fn build_api_request(
    api_client: reqwest::Client,
    root: &str,
    endpoint: &str,
    query: Vec<(String, String)>,
) -> ApiRequest {
    ApiRequest::builder()
        .client(api_client)
        .root(root)
        .endpoint(endpoint)
        .query(query)
        .build()
}

/// Parses the `serde_json::Value` into a given Type T
/// TODO: Implement Error handling for Serde, ResponderError, ProcessingError
#[allow(dead_code)]
pub(crate) fn parse_into<T, E>(val: &serde_json::Value) -> Result<T, E>
where
    T: for<'de> serde::Deserialize<'de>,
    E: Error,
{
    Ok(serde_json::from_str::<T>(
        &serde_json::to_string(&val).expect("Serialisation to String failed."),
    )
    .expect("Deserialisation to Type T failed."))
}
