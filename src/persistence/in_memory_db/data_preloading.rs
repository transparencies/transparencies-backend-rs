//! Everything around preloading data in another thread for future use within
//! our in-memory DB implemented by `Arc<Mutex<T>>`
use std::{
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use aoe2net::endpoints::strings::GetApiStringsRequest;
use api_client::error::ClientRequestError;
use dashmap::DashMap;
use serde_json::Value as JsonValue;
use tokio::{
    sync::Mutex,
    time,
};
use tracing::warn;
use url::Url;
use uuid::Uuid;

// STATICS USED
use crate::{
    domain::{
        api_handler::client::A2NClient,
        types::{
            aoc_ref::{
                AoePlatforms,
                AoePlayers,
                AoeTeams,
            },
            error::{
                ApiRequestError,
                FileRequestError,
            },
            requests::{
                File,
                FileFormat,
                GithubFileRequest,
            },
            InMemoryDb,
        },
        util,
    },
    APP_USER_AGENT,
    CLIENT_CONNECTION_TIMEOUT,
    CLIENT_REQUEST_TIMEOUT,
    GAME_STRINGS,
    LANGUAGE_STRINGS,
};

/// Gets all of our static data in a separated thread
///
/// # Usage
/// When wanting to `export` data for offline usage set only ONE of
/// * `export_path` - a string slice holding the path where to write the offline
///   data to
///
/// otherwhise `export_path` should be empty: `""`.
///
/// # Arguments
/// * `git_client_clone` - a [`reqwest::Client`] clone for connection pooling
///   purposes separated for Github root
/// * `aoe2net_client_clone` - a [`reqwest::Client`] clone for connection
///   pooling purposes separated for AoE2.net root
/// * `in_memory_db_clone` - an [`InMemoryDb`] that is wrapped by [`Arc`] and
///   guarded by a [`Mutex`]
///
/// # Errors
/// This functions doesn't error out or returns a Result, but it throws a
/// warning in console if the process experienced an error.
///
/// # Panics
/// This function shouldn't panic.
pub async fn get_static_data_inside_thread(in_memory_db_clone: Arc<Mutex<InMemoryDb>>,
                                           github_root: Url,
                                           aoe2_net_root: Url) {
    let background_client =
        reqwest::Client::builder().user_agent(*APP_USER_AGENT)
                                  .timeout(*CLIENT_REQUEST_TIMEOUT)
                                  .connect_timeout(*CLIENT_CONNECTION_TIMEOUT)
                                  .use_rustls_tls()
                                  .https_only(true)
                                  .build()
                                  .unwrap();

    tokio::spawn(async move {
        loop {
            match preload_data(Some(background_client.clone()),
                               Some(background_client.clone()),
                               in_memory_db_clone.clone(),
                               github_root.clone(),
                               aoe2_net_root.clone(),
                               None,
                               false).await
            {
                Ok(_) => {},
                Err(e) => {
                    warn!("Threaded data pulling experienced an error: {:#?}",
                          e);
                },
            }

            time::sleep(Duration::from_secs(600)).await;
        }
    });
}

/// Preload data from `aoe2net` and `Github`
///
/// # Arguments
/// TODO
///
/// # Example
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     use std::sync::Arc;
///
///     use tokio::sync::Mutex;
///     use transparencies_backend_rs::{
///         domain::types::{
///             requests::ApiClient,
///             InMemoryDb,
///         },
///         persistence::in_memory_db::data_preloading::preload_data,
///     };
///     use url::Url;
///
///     let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
///     let request_client = reqwest::Client::default();
///
///     let github_url =
///         Url::parse("https://raw.githubusercontent.com").unwrap();
///     let aoe2_net_url = Url::parse("https://aoe2.net/api").unwrap();
///
///     preload_data(Some(request_client.clone()),
///                  Some(request_client.clone()),
///                  in_memory_db.clone(),
///                  github_url,
///                  aoe2_net_url,
///                  None,
///                  false).await
///                        .unwrap();
/// }
/// ```
///
/// # Errors
// TODO: Better error handling, how should we deal with it, if one of these
// doesn't work or get parsed correctly?
#[tracing::instrument(
    name = "Preloading data ...",
    skip(api_client, git_client, in_memory_db, github_root, aoe2_net_root, export_path),
    fields(
task_id = %Uuid::new_v4(),
mocking_enabled = %mocking,
)
)]
pub async fn preload_data(api_client: Option<reqwest::Client>,
                          git_client: Option<reqwest::Client>,
                          in_memory_db: Arc<Mutex<InMemoryDb>>,
                          github_root: Url,
                          aoe2_net_root: Url,
                          export_path: Option<PathBuf>,
                          mocking: bool)
                          -> Result<(), ApiRequestError> {
    preload_aoc_ref_data(git_client.map_or(reqwest::Client::default(),
                                           |client| client),
                         in_memory_db.clone(),
                         github_root,
                         export_path.clone().map(|mut path| {
                                                path.push("ref-data");
                                                path
                                            }),
                         mocking).await
                                 .expect("Unable to preload files from Github");

    index_aoc_ref_data(in_memory_db.clone()).await;

    preload_aoe2_net_data(
        api_client.map_or(reqwest::Client::default(), |client| client),
        in_memory_db.clone(),
        aoe2_net_root,
        export_path.map(|mut path| {
            path.push("languages");
            path
        }),
    )
    .await
    .expect("Unable to preload data from AoE2.net");

    Ok(())
}

/// Index the `player_ids` of Players in the `players.yaml` file of
/// aoc-reference-data repository in a [`dashmap::DashMap`] to make them be
/// easily looked-up during the processing stage
// TODO: Handle Result better for indexing errors
#[allow(unused_must_use)]
async fn index_aoc_ref_data(in_memory_db: Arc<Mutex<InMemoryDb>>) {
    {
        let mut guard = in_memory_db.lock().await;
        guard.github_file_content.index().map_err(|errs| {
                                             errs.into_iter().map(|err| {
                warn!(
                    "Indexing of player aliases threw an error: {:#?}\n",
                    err
                );
            })
                                         });
    }
}

/// Preload data from `aoe2net`
///
/// # Errors
// TODO
/// # Panics
// TODO
pub async fn preload_aoe2_net_data(api_client: reqwest::Client,
                                   in_memory_db: Arc<Mutex<InMemoryDb>>,
                                   root: Url,
                                   export_path: Option<PathBuf>)
                                   -> Result<(), ApiRequestError> {
    let language_requests = build_language_requests(&root);

    let responses = assemble_languages_to_dashmap(api_client,
                                                  language_requests,
                                                  export_path).await?;

    {
        let mut guard = in_memory_db.lock().await;
        guard.aoe2net_languages = responses;
    }

    Ok(())
}

/// Pull responses for `language strings` into a [`dashmap::DashMap`] for being
/// easily looked-up later on
///
/// # Errors
// TODO
async fn assemble_languages_to_dashmap(
    api_client: reqwest::Client,
    language_requests: Vec<(String, GetApiStringsRequest<'_>)>,
    export_path: Option<PathBuf>)
    -> Result<DashMap<String, JsonValue>, ClientRequestError<reqwest::Error>> {
    let responses: DashMap<String, JsonValue> =
        DashMap::with_capacity(LANGUAGE_STRINGS.len());

    let client = A2NClient::with_client(api_client);

    for (req_name, req) in language_requests {
        let response = client.req_get(req).await?;
        let data = response.data.unwrap_or_default();

        responses.insert(req_name.to_string(), data.clone());

        if export_path.is_some() {
            util::export_to_json(&File { name: req_name.to_string(),
                                         ext: FileFormat::Json },
                                 &export_path.clone().unwrap(),
                                 &data)
        }
    }

    Ok(responses)
}

/// Builds all requests for the `LANGUAGE_STRINGS`
fn build_language_requests(_root: &Url) -> Vec<(String, GetApiStringsRequest)> {
    let mut language_requests = Vec::with_capacity(LANGUAGE_STRINGS.len());

    // Build requests for each `GAME_STRING` with each `LANGUAGE_STRING`
    for game in &(*GAME_STRINGS) {
        for language in &(*LANGUAGE_STRINGS) {
            language_requests.push((
                (*language).to_string(),
                GetApiStringsRequest::builder()
                    .game(*game)
                    .language(*language)
                    .build(),
            ));
        }
    }

    language_requests
}

/// Preload data from `aoc-reference-data` Github repository
///
/// # Errors
// TODO
/// # Panics
// TODO
pub async fn preload_aoc_ref_data(git_client: reqwest::Client,
                                  in_memory_db: Arc<Mutex<InMemoryDb>>,
                                  root: Url,
                                  export_path: Option<PathBuf>,
                                  mocking: bool)
                                  -> Result<(), FileRequestError> {
    let files = get_github_file_list();

    let mut ref_data_repository = root.clone();
    ref_data_repository
        .set_path("SiegeEngineers/aoc-reference-data/master/data/");

    for file in files {
        let file_path = ref_data_repository.join(&file.display())?;

        let req = util::build_github_request(git_client.clone(), file_path);

        let response: String = req.execute().await?.text().await?;

        assemble_data_to_db(file,
                            in_memory_db.clone(),
                            response,
                            req,
                            export_path.clone(),
                            mocking).await?;
    }

    Ok(())
}

/// Parses the responses from a `request::Response` type and writes the Result
/// into the in-memory database
async fn assemble_data_to_db(file: File,
                             in_memory_db: Arc<Mutex<InMemoryDb>>,
                             response: String,
                             req: GithubFileRequest,
                             export_path: Option<PathBuf>,
                             mocking: bool)
                             -> Result<(), FileRequestError> {
    match file.ext() {
        FileFormat::Json => match file.name().as_str() {
            "platforms" => {
                if export_path.is_some() {
                    util::export_to_json(
                        &file,
                        &export_path.clone().unwrap(),
                        &serde_json::from_str::<JsonValue>(&response)?,
                    )
                };

                let mut guard = in_memory_db.lock().await;
                guard.github_file_content.platforms =
                    serde_json::from_str::<AoePlatforms>(&response)?;
            }
            "teams" => {
                if export_path.is_some() {
                    util::export_to_json(
                        &file,
                        &export_path.clone().unwrap(),
                        &serde_json::from_str::<JsonValue>(&response)?,
                    )
                };

                let mut guard = in_memory_db.lock().await;
                guard.github_file_content.teams =
                    serde_json::from_str::<AoeTeams>(&response)?;
            }
            _ => {
                return Err(FileRequestError::RequestNotMatching {
                    name: file.name().to_string(),
                    req: req.clone(),
                })
            }
        },
        FileFormat::Yaml => {
            if let "players" = file.name().as_str() {
                let deserialized =
                    serde_yaml::from_str::<AoePlayers>(&response)?;

                if export_path.is_none() {
                    let mut guard = in_memory_db.lock().await;
                    guard.github_file_content.players = deserialized.clone();
                }
                else if mocking {
                    // ATTENTION! Mocking is enabled, we don't want to use
                    // `yaml` for the players file but imitate it. This means
                    // that the mocking server is delivering a `json`-file under
                    // the same filename `players.yaml` for convenience.
                    let mut guard = in_memory_db.lock().await;
                    guard.github_file_content.players =
                        serde_json::from_str::<AoePlayers>(&response)?;
                }
                else {
                    util::export_to_json(
                        &file,
                        &export_path.unwrap(),
                        &serde_yaml::from_str(&response)?,
                    );

                    let mut guard = in_memory_db.lock().await;
                    guard.github_file_content.players = deserialized.clone();
                }
            }
            else {
                return Err(FileRequestError::RequestNotMatching {
                    name: file.name().to_string(),
                    req: req.clone(),
                });
            }
        }

        _ => {
            return Err(FileRequestError::RequestNotMatching {
                name: file.name().to_string(),
                req: req.clone(),
            })
        }
    }

    Ok(())
}

/// Create a list of files that need to be downloaded from github repository
#[must_use]
pub fn get_github_file_list() -> Vec<File> {
    vec![File { name: "platforms".to_string(),
                ext: FileFormat::Json },
         File { name: "teams".to_string(),
                ext: FileFormat::Json },
         File { name: "players".to_string(),
                ext: FileFormat::Yaml },]
}
