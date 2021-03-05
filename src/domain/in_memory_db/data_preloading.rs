//! Everything around preloading data in another thread for future use within
//! our in-memory DB implemented by `Arc<Mutex<T>>`

use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::Mutex;
use tracing::warn;

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
/// Can be used later also for adding AoE3DE and/or AoE4 support
pub(crate) static GAME_STRINGS: [&str; 1] = ["aoe2de"];

/// Preload data from `aoe2net` and `Github`
// TODO: Better error handling, how should we deal with it, if one of these
// doesn't work or get parsed correctly?
pub async fn preload_data(
    api_client: reqwest::Client,
    git_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<(), ApiRequestError> {
    preload_aoc_ref_data(git_client.clone(), in_memory_db.clone())
        .await
        .expect("Unable to preload files from Github");

    index_aoc_ref_data(in_memory_db.clone()).await;

    preload_aoe2_net_data(api_client.clone(), in_memory_db.clone())
        .await
        .expect("Unable to preload data from AoE2.net");

    Ok(())
}

/// Index the `player_ids` of Players in the `players.yaml` file of
/// aoc-reference-data repository in a HashMap to make them be easily looked-up
/// during the processing stage
// TODO: Handle Result better for indexing errors
#[allow(unused_must_use)]
async fn index_aoc_ref_data(in_memory_db: Arc<Mutex<InMemoryDb>>) {
    {
        let mut guard = in_memory_db.lock().await;
        guard.github_file_content.index().map_err(|errs| {
            errs.into_iter().map(|err| {
                warn!("Indexing of player aliases threw an error: {:#?}\n", err)
            })
        });
    }
}

/// Preload data from `aoe2net`
pub async fn preload_aoe2_net_data(
    api_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<(), ApiRequestError> {
    let language_requests = assemble_language_requests(&api_client);

    let responses =
        load_language_responses_into_hashmap(language_requests).await?;

    {
        let mut guard = in_memory_db.lock().await;
        guard.aoe2net_languages = responses;
    }

    Ok(())
}

/// Pull responses for `language strings` into a HashMap for being easily
/// looked-up later on
async fn load_language_responses_into_hashmap(
    language_requests: Vec<(&str, ApiRequest)>
) -> Result<HashMap<&str, serde_json::Value>, ApiRequestError> {
    let mut responses: HashMap<&str, serde_json::Value> =
        HashMap::with_capacity(LANGUAGE_STRINGS.len());

    for (_req_number, (req_name, req)) in language_requests.iter().enumerate() {
        responses.insert(req_name, req.execute().await?);
    }

    Ok(responses)
}

/// Builds all requests for the `LANGUAGE_STRINGS`
fn assemble_language_requests(
    api_client: &reqwest::Client
) -> Vec<(&'static str, ApiRequest)> {
    let mut language_requests: Vec<(&str, ApiRequest)> =
        Vec::with_capacity(LANGUAGE_STRINGS.len());

    // Build requests for each `GAME_STRING` with each `LANGUAGE_STRING`
    for game in &GAME_STRINGS {
        for language in &LANGUAGE_STRINGS {
            language_requests.push((
                language,
                util::build_api_request(
                    api_client.clone(),
                    "https://aoe2.net/api",
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
pub async fn preload_aoc_ref_data(
    git_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<(), FileRequestError> {
    let files = create_github_file_list();

    for file in files {
        let req = util::build_github_request(
            git_client.clone(),
            "https://raw.githubusercontent.com",
            "SiegeEngineers",
            "aoc-reference-data",
            "master/data",
            &file,
        );

        let response = req.execute().await?;

        update_data_in_db(file, in_memory_db.clone(), response, req).await?;
    }

    Ok(())
}

/// Parses the responses from a `request::Response` type and writes the Result
/// into the in-memory database
async fn update_data_in_db(
    file: File,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    response: reqwest::Response,
    req: GithubFileRequest,
) -> Result<(), FileRequestError> {
    match file.ext() {
        FileFormat::Json => match file.name().as_str() {
            "platforms" => {
                let mut guard = in_memory_db.lock().await;
                guard.github_file_content.platforms =
                    response.json::<AoePlatforms>().await?
            }
            "teams" => {
                let mut guard = in_memory_db.lock().await;
                guard.github_file_content.teams =
                    response.json::<AoeTeams>().await?
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
                let mut guard = in_memory_db.lock().await;
                guard.github_file_content.players =
                    serde_yaml::from_slice::<AoePlayers>(
                        &response.bytes().await?,
                    )
                    .unwrap()
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
fn create_github_file_list() -> Vec<File> {
    let mut files: Vec<File> = Vec::with_capacity(3);

    files.push(File {
        name: "players".to_string(),
        ext: FileFormat::Yaml,
    });
    files.push(File {
        name: "platforms".to_string(),
        ext: FileFormat::Json,
    });
    files.push(File {
        name: "teams".to_string(),
        ext: FileFormat::Json,
    });

    files
}
