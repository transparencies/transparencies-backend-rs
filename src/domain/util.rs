//! Additional utility functions that are useful in different modules

use std::{
    error::Error,
    fs,
    io::BufWriter,
    path::{
        Path,
        PathBuf,
    },
};

use crate::domain::types::{
    requests::FileFormat,
    ApiRequest,
    File,
    GithubFileRequest,
};

use url::Url;

/// Assembles a request for a file in a Github repository
pub(crate) fn build_github_request(
    git_client: reqwest::Client,
    url: Url,
) -> GithubFileRequest {
    GithubFileRequest::builder()
        .client(git_client)
        .url(url)
        .build()
}

/// Assembles a `GET` request for an API
/// Refactoring: Use this function
pub(crate) fn build_api_request(
    api_client: reqwest::Client,
    root: Url,
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

/// Parses the [`serde_json::Value`] into a given `type T`
// TODO: Implement Error handling for [`serde_json::Error`],
// [`ResponderError`], [`ProcessingError`]
#[allow(dead_code, clippy::unnecessary_wraps)]
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
/// Write the data in JSON format to a file for debugging purposes
///
/// # Arguments
/// * `file` - holds a [`File`] with `Filename` and a file extension as
///   [`FileFormat`]
/// * `path` - holding a [`PathBuf`] with the output path
/// * `data` - a [`serde_json::Value`] that holds the `data` that need to be
///   serialized
///  
/// # Panics
/// Will panic if data cannot be written or file can not be created in
/// Filesystem
pub(crate) fn export_to_json(
    file: &File,
    path: &Path,
    data: &serde_json::Value,
) {
    let json_file = if file.ext().is_json() {
        file.clone()
    }
    else {
        File {
            name: (file.name()).to_string(),
            ext: FileFormat::Json,
        }
    };
    let mut assembly_path = PathBuf::new();
    assembly_path.push(path);
    assembly_path.push(format!("{}", json_file));

    // Open the file in writable mode with buffer.
    let file = fs::File::create(assembly_path.as_os_str())
        .expect("Couldn't create file.");
    let mut writer = BufWriter::new(file);

    // Write data to file
    serde_json::to_writer_pretty(&mut writer, &data)
        .expect("Wrting data to file experienced an error.");
}

/// Extracts the filename without extension of a Path
///
/// # Arguments
// TODO
/// # Errors
// TODO
/// # Panics
// TODO
#[must_use]
pub fn extract_filename(path: &Path) -> String {
    let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
    let name: Vec<&str> = file_name.split(".json").collect();
    name[0].to_string()
}
