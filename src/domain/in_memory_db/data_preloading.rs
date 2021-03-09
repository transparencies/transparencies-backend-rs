//! Everything around preloading data in another thread for future use within
//! our in-memory DB implemented by `Arc<Mutex<T>>`
use dashmap::DashMap;

use std::{
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::Mutex,
    time,
};

use tracing::warn;
use url::Url;

use crate::domain::{
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
            ApiRequest,
            File,
            FileFormat,
            GithubFileRequest,
        },
        InMemoryDb,
    },
    util,
};

/// All of the current `language strings` of the AoE2.net API
/// used for preloading the Language files
pub(crate) static LANGUAGE_STRINGS: [&str; 18] = [
    "en", "de", "el", "es", "es-MX", "fr", "hi", "it", "ja", "ko", "ms", "nl",
    "pt", "ru", "tr", "vi", "zh", "zh-TW",
];

/// `Game strings` used for preloading and other request towards the AoE2.net
/// API.
/// Can be used later also for adding `AoE3DE` and/or `AoE4` support
pub(crate) static GAME_STRINGS: [&str; 1] = ["aoe2de"];

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
pub async fn get_static_data_inside_thread(
    git_client_clone: reqwest::Client,
    aoe2net_client_clone: reqwest::Client,
    in_memory_db_clone: Arc<Mutex<InMemoryDb>>,
    github_root: Url,
    aoe2_net_root: Url,
) {
    tokio::spawn(async move {
        loop {
            match preload_data(
                Some(git_client_clone.clone()),
                Some(aoe2net_client_clone.clone()),
                in_memory_db_clone.clone(),
                github_root.clone(),
                aoe2_net_root.clone(),
                None,
                false,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "Threaded data pulling experienced an error: {:#?}",
                        e
                    );
                }
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
///     use tokio::sync::Mutex;
///     use transparencies_backend_rs::domain::{
///         in_memory_db::data_preloading::preload_data,
///         types::{
///             requests::ApiClient,
///             InMemoryDb,
///         },
///     };
///     use url::Url;
///
///     let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
///     let api_clients = ApiClient::default();
///     let github_url =
///         Url::parse("https://raw.githubusercontent.com").unwrap();
///     let aoe2_net_url = Url::parse("https://aoe2.net/api").unwrap();
///
///     preload_data(
///         Some(api_clients.github.clone()),
///         Some(api_clients.aoe2net.clone()),
///         in_memory_db.clone(),
///         github_url,
///         aoe2_net_url,
///         None,
///         false,
///     )
///     .await
///     .unwrap()
/// }
/// ```
///
/// # Errors
// TODO: Better error handling, how should we deal with it, if one of these
// doesn't work or get parsed correctly?
pub async fn preload_data(
    api_client: Option<reqwest::Client>,
    git_client: Option<reqwest::Client>,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    github_root: Url,
    aoe2_net_root: Url,
    export_path: Option<&str>,
    mocking: bool,
) -> Result<(), ApiRequestError> {
    let aoc_folder = if let Some(path) = export_path {
        format!("{}{}", path, "/ref-data/")
    }
    else {
        "".to_string()
    };

    let aoe2net_language_folder = if let Some(path) = export_path {
        format!("{}{}", path, "/languages/")
    }
    else {
        "".to_string()
    };

    preload_aoc_ref_data(
        git_client.map_or(reqwest::Client::default(), |client| client),
        in_memory_db.clone(),
        github_root,
        &aoc_folder,
        mocking,
    )
    .await
    .expect("Unable to preload files from Github");

    index_aoc_ref_data(in_memory_db.clone()).await;

    preload_aoe2_net_data(
        api_client.map_or(reqwest::Client::default(), |client| client),
        in_memory_db.clone(),
        aoe2_net_root,
        &aoe2net_language_folder,
    )
    .await
    .expect("Unable to preload data from AoE2.net");

    Ok(())
}

/// Index the `player_ids` of Players in the `players.yaml` file of
/// aoc-reference-data repository in a [`HashMap`] to make them be easily
/// looked-up during the processing stage
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
pub async fn preload_aoe2_net_data(
    api_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    root: Url,
    export_path: &str,
) -> Result<(), ApiRequestError> {
    let language_requests = assemble_language_requests(&api_client, &root);

    let responses =
        load_language_responses_into_dashmap(language_requests, export_path)
            .await?;

    {
        let mut guard = in_memory_db.lock().await;
        guard.aoe2net_languages = responses;
    }

    Ok(())
}

/// Pull responses for `language strings` into a [`HashMap`] for being easily
/// looked-up later on
///
/// # Errors
// TODO
async fn load_language_responses_into_dashmap(
    language_requests: Vec<(String, ApiRequest)>,
    export_path: &str,
) -> Result<DashMap<String, serde_json::Value>, ApiRequestError> {
    let responses: DashMap<String, serde_json::Value> =
        DashMap::with_capacity(LANGUAGE_STRINGS.len());

    for (req_name, req) in language_requests {
        let response: serde_json::Value = req.execute().await?;
        responses.insert(req_name.to_string(), response.clone());

        if !export_path.is_empty() {
            util::export_to_json(
                &File {
                    name: req_name.to_string(),
                    ext: FileFormat::Json,
                },
                &PathBuf::from_str(export_path).unwrap(),
                &response,
            )
        }
    }

    Ok(responses)
}

/// Builds all requests for the `LANGUAGE_STRINGS`
fn assemble_language_requests(
    api_client: &reqwest::Client,
    root: &Url,
) -> Vec<(String, ApiRequest)> {
    let mut language_requests: Vec<(String, ApiRequest)> =
        Vec::with_capacity(LANGUAGE_STRINGS.len());

    // Build requests for each `GAME_STRING` with each `LANGUAGE_STRING`
    for game in &GAME_STRINGS {
        for language in &LANGUAGE_STRINGS {
            language_requests.push((
                (*language).to_string(),
                util::build_api_request(
                    api_client.clone(),
                    root.clone(),
                    "strings",
                    vec![
                        ("game".to_string(), (*game).to_string()),
                        ("language".to_string(), (*language).to_string()),
                    ],
                ),
            ));
        }
    }

    language_requests
}

/// Preload data from `aoc-reference-data` Github repository
///
/// # Errors
// TODO
pub async fn preload_aoc_ref_data(
    git_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    root: Url,
    export_path: &str,
    mocking: bool,
) -> Result<(), FileRequestError> {
    let files = create_github_file_list();

    let mut ref_data_repository = root.clone();
    ref_data_repository
        .set_path("SiegeEngineers/aoc-reference-data/master/data/");

    for file in files {
        let file_path = ref_data_repository.join(&file.display())?;

        let req = util::build_github_request(git_client.clone(), file_path);

        let response: String = req.execute().await?.text().await?;

        update_data_in_db(
            file,
            in_memory_db.clone(),
            response,
            req,
            export_path,
            mocking,
        )
        .await?;
    }

    Ok(())
}

/// Parses the responses from a `request::Response` type and writes the Result
/// into the in-memory database
async fn update_data_in_db(
    file: File,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    response: String,
    req: GithubFileRequest,
    export_path: &str,
    mocking: bool,
) -> Result<(), FileRequestError> {
    match file.ext() {
        FileFormat::Json => match file.name().as_str() {
            "platforms" => {
                if !export_path.is_empty() {
                    util::export_to_json(
                        &file,
                        &PathBuf::from_str(export_path).unwrap(),
                        &serde_json::from_str::<serde_json::Value>(&response)?,
                    )
                };

                let mut guard = in_memory_db.lock().await;
                guard.github_file_content.platforms =
                    serde_json::from_str::<AoePlatforms>(&response)?;
            }
            "teams" => {
                if export_path.is_empty() {
                    util::export_to_json(
                        &file,
                        &PathBuf::from_str(export_path).unwrap(),
                        &serde_json::from_str::<serde_json::Value>(&response)?,
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

                if export_path.is_empty() {
                    let mut guard = in_memory_db.lock().await;
                    guard.github_file_content.players = deserialized.clone();

                    // serde_yaml::from_slice::<AoePlayers>(
                    //     &response.inner().bytes().await?,
                    // )
                    // .unwrap()
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
                        &PathBuf::from_str(export_path).unwrap(),
                        &serde_yaml::from_str(&response)?,
                        /* &serde_yaml::from_slice(
                         *     &response.inner().bytes().await?,
                         * )
                         * .unwrap(), */
                    );

                    let mut guard = in_memory_db.lock().await;
                    guard.github_file_content.players = deserialized.clone();
                    // serde_yaml::from_slice::<AoePlayers>(
                    //     &response.inner().bytes().await?,
                    // )
                    // .unwrap();
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
pub fn create_github_file_list() -> Vec<File> {
    vec![
        File {
            name: "platforms".to_string(),
            ext: FileFormat::Json,
        },
        File {
            name: "teams".to_string(),
            ext: FileFormat::Json,
        },
        File {
            name: "players".to_string(),
            ext: FileFormat::Yaml,
        },
    ]
}
