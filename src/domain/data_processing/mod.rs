pub mod error;
mod match_data_responder;
pub mod match_info_processor;
use match_info_processor::MatchInfoProcessor;

use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value;

use crate::domain::{
    api_handler::client::{
        APP_USER_AGENT,
        CLIENT_CONNECTION_TIMEOUT,
        CLIENT_REQUEST_TIMEOUT,
    },
    types::{
        aoc_ref::{
            platforms,
            platforms::PlatformsList,
            players,
            players::PlayersList,
            teams,
            teams::TeamsList,
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
use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use stable_eyre::eyre::{
    Report,
    Result,
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

use crate::domain::data_processing::error::{
    FileRequestError,
    ProcessingError,
};

/// Download static files continously every 10 minutes inside a thread
pub fn get_static_files_inside_thread(
    git_client_clone: reqwest::Client,
    aoc_reference_data_clone: Arc<Mutex<RefDataLists>>,
) {
    tokio::spawn(async move {
        loop {
            load_aoc_ref_data(
                git_client_clone.clone(),
                aoc_reference_data_clone.clone(),
            )
            .await
            .expect("Unable to load Files from Github");

            time::sleep(Duration::from_secs(600)).await;
        }
    });
}

/// Entry point for processing part of `matchinfo` endpoint
pub async fn process_match_info_request(
    par: MatchInfoRequest,
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
    // ) -> Result<MatchInfoResult, ProcessingError> {
) -> Result<(), ProcessingError> {
    debug!(
        "MatchInfoRequest: {:?} with {:?}",
        par.id_type, par.id_number
    );

    let responses =
        MatchDataResponses::new_with_match_data(par, client, ref_data).await?;

    // Debugging
    responses.export_data_to_file();

    // let result = MatchInfoProcessor::new_with_response(responses)?
    //     .process()?
    //     .assemble()?;

    // Ok(result)
    Ok(())
}

pub async fn load_aoc_ref_data(
    git_client: reqwest::Client,
    reference_db: Arc<Mutex<RefDataLists>>,
) -> Result<(), FileRequestError> {
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

    for file in files {
        let req = GithubFileRequest::builder()
            .client(git_client.clone())
            .root("https://raw.githubusercontent.com")
            .user("SiegeEngineers")
            .repo("aoc-reference-data")
            .uri("master/data")
            .file(file.clone())
            .build();

        let response = req.execute().await?;

        match file.ext() {
            FileFormat::Json => match file.name().as_str() {
                "platforms" => {
                    let mut locked = reference_db.lock().await;
                    locked.platforms =
                        response.json::<Vec<platforms::Platforms>>().await?
                }
                "teams" => {
                    let mut locked = reference_db.lock().await;
                    locked.teams = response.json::<Vec<teams::Teams>>().await?
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
                    let mut locked = reference_db.lock().await;
                    locked.players =
                        serde_yaml::from_slice::<Vec<players::Player>>(
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
    }

    Ok(())
}
