use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use stable_eyre::eyre::{
    Report,
    WrapErr,
};

use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::{
    io::AsyncReadExt,
    sync::Mutex,
    time::{
        self,
        Duration,
    },
};

use crate::domain::{
    api_handler::client::{
        APP_USER_AGENT,
        CLIENT_CONNECTION_TIMEOUT,
        CLIENT_REQUEST_TIMEOUT,
    },
    data_processing::error::{
        ApiRequestError,
        FileRequestError,
        IndexingError,
        ProcessingError,
    },
    types::{
        aoc_ref::{
            AoePlatforms,
            AoePlayers,
            AoeTeams,
            RefDataLists,
        },
        api::{
            match_info_response::*,
            MatchInfoRequest,
            MatchInfoResult,
        },
        requests::{
            ApiRequest,
            File,
            FileFormat,
            GithubFileRequest,
        },
        InMemoryDb,
        MatchDataResponses,
    },
};

pub(crate) static LANGUAGE_STRINGS: [&str; 18] = [
    "en", "de", "el", "es", "es-MX", "fr", "hi", "it", "ja", "ko", "ms", "nl",
    "pt", "ru", "tr", "vi", "zh", "zh-TW",
];

pub(crate) static GAME_STRINGS: [&str; 1] = ["aoe2de"];

/// Preload data from `aoe2net`
pub async fn preload_data(
    api_client: reqwest::Client,
    git_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<(), ApiRequestError> {
    preload_aoc_ref_data(git_client.clone(), in_memory_db.clone())
        .await
        .expect("Unable to preload files from Github");

    index_aoc_ref_data(in_memory_db.clone())
        .await
        .expect("Indexing of players failed.");

    preload_aoe2_net_data(api_client.clone(), in_memory_db.clone())
        .await
        .expect("Unable to preload data from AoE2.net");

    Ok(())
}

async fn index_aoc_ref_data(
    in_memory_db: Arc<Mutex<InMemoryDb>>
) -> Result<(), IndexingError> {
    {
        let mut guard = in_memory_db.lock().await;
        guard.github_file_content.index()?;
    }

    Ok(())
}

/// Preload data from `aoe2net`
pub async fn preload_aoe2_net_data(
    api_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<(), ApiRequestError> {
    let language_requests = assemble_language_requests(api_client);

    let responses =
        load_language_responses_into_hashmap(language_requests).await?;

    {
        let mut guard = in_memory_db.lock().await;
        guard.aoe2net_languages = responses;
    }

    Ok(())
}

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

/// Assembles all requests for the `LANGUAGE_STRINGS`
fn assemble_language_requests(
    api_client: reqwest::Client
) -> Vec<(&'static str, ApiRequest)> {
    let mut language_requests: Vec<(&str, ApiRequest)> =
        Vec::with_capacity(LANGUAGE_STRINGS.len());

    // Build requests for each `GAME_STRING` with each `LANGUAGE_STRING`
    for game in GAME_STRINGS.iter() {
        for language in LANGUAGE_STRINGS.iter() {
            language_requests.push((
                language,
                ApiRequest::builder()
                    .client(api_client.clone())
                    .root("https://aoe2.net/api")
                    .endpoint("strings")
                    .query(vec![
                        ("game".to_string(), game.to_string()),
                        ("language".to_string(), language.to_string()),
                    ])
                    .build(),
            ))
        }
    }

    language_requests
}

/// Preload data from `aoc-reference-data` Github repository
pub async fn preload_aoc_ref_data(
    git_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<(), FileRequestError> {
    let files = create_file_list();
    let par: [&str; 4] = [
        "https://raw.githubusercontent.com",
        "SiegeEngineers",
        "aoc-reference-data",
        "master/data",
    ];

    for file in files {
        let req = assemble_github_request(
            git_client.clone(),
            par[0],
            par[1],
            par[2],
            par[3],
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

/// Assembles a request for the `aoc-reference-data` Github repository
fn assemble_github_request(
    git_client: reqwest::Client,
    root: &str,
    user: &str,
    repo: &str,
    uri: &str,
    file: &File,
) -> GithubFileRequest {
    let req = GithubFileRequest::builder()
        .client(git_client.clone())
        .root(root)
        .user(user)
        .repo(repo)
        .uri(uri)
        .file(file.clone())
        .build();

    req
}

/// Assembles a get request for an API
fn assemble_api_request(
    api_client: reqwest::Client,
    root: &str,
    endpoint: &str,
    query: Vec<(String, String)>,
) -> ApiRequest {
    let req = ApiRequest::builder()
        .client(api_client.clone())
        .root(root)
        .endpoint(endpoint)
        .query(query)
        .build();

    req
}

/// Create a list of files that need to be downloaded from github repository
fn create_file_list() -> Vec<File> {
    let mut files: Vec<File> = Vec::with_capacity(3);
    files.push(
        File::builder()
            .name("players")
            .ext(FileFormat::Yaml)
            .build(),
    );

    files.push(
        File::builder()
            .name("platforms")
            .ext(FileFormat::Json)
            .build(),
    );

    files.push(File::builder().name("teams").ext(FileFormat::Json).build());

    files
}
