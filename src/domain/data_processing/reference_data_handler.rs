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
        MatchDataResponses,
    },
};

use crate::domain::data_processing::error::{
    FileRequestError,
    ProcessingError,
};

/// Download data from `aoc-reference-data` Github repository
pub async fn load_aoc_ref_data(
    git_client: reqwest::Client,
    reference_db: Arc<Mutex<RefDataLists>>,
) -> Result<(), FileRequestError> {
    let files = create_file_list();

    for file in files {
        let req = assemble_github_request(git_client.clone(), &file);

        let response = req.execute().await?;

        update_data_in_db(file, reference_db.clone(), response, req).await?;
    }

    Ok(())
}

/// Parses the responses from a `request::Response` type and writes the Result
/// into the in-memory database
async fn update_data_in_db(
    file: File,
    reference_db: Arc<Mutex<RefDataLists>>,
    response: reqwest::Response,
    req: GithubFileRequest,
) -> Result<(), FileRequestError> {
    match file.ext() {
        FileFormat::Json => match file.name().as_str() {
            "platforms" => {
                let mut guard = reference_db.lock().await;
                guard.platforms = response.json::<AoePlatforms>().await?
            }
            "teams" => {
                let mut guard = reference_db.lock().await;
                guard.teams = response.json::<AoeTeams>().await?
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
                let mut guard = reference_db.lock().await;
                guard.players = serde_yaml::from_slice::<AoePlayers>(
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
    file: &File,
) -> GithubFileRequest {
    let req = GithubFileRequest::builder()
        .client(git_client.clone())
        .root("https://raw.githubusercontent.com")
        .user("SiegeEngineers")
        .repo("aoc-reference-data")
        .uri("master/data")
        .file(file.clone())
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
