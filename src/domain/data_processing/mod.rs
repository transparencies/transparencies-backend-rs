pub mod error;
mod match_data_responder;
pub mod match_info_processor;
use match_info_processor::MatchInfoProcessor;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::domain::{
    api_handler::client::{
        APP_USER_AGENT, CLIENT_CONNECTION_TIMEOUT, CLIENT_REQUEST_TIMEOUT,
    },
    types::{
        aoc_ref::{
            platforms, platforms::PlatformsList, players, players::PlayersList,
            teams, teams::TeamsList, RefDataLists,
        },
        api::{match_info_response::*, MatchInfoRequest, MatchInfoResult},
        requests::{ApiRequest, File, FileFormat, GithubFileRequest},
        MatchDataResponses,
    },
};
use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{Report, Result, WrapErr};

use std::{sync::Arc, time::Duration};

use crate::domain::data_processing::error::ProcessingError;
use std::collections::HashMap;

/// Entry point for processing part of `matchinfo` endpoint
pub async fn process_match_info_request(
    par: MatchInfoRequest,
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> Result<MatchInfoResult, ProcessingError> {
    debug!(
        "MatchInfoRequest: {:?} with {:?}",
        par.id_type, par.id_number
    );

    let responses =
        MatchDataResponses::new_with_match_data(par, client, ref_data).await?;

    responses.export_data_to_file();

    let result = MatchInfoProcessor::new_with_response(responses)?
        .process()?
        .assemble()?;

    Ok(result)
}

pub async fn load_aoc_ref_data(
    git_client: reqwest::Client,
    reference_db: Arc<Mutex<RefDataLists>>,
) -> Result<()> {
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
        let request: Option<GithubFileRequest> = Some(
            GithubFileRequest::builder()
                .client(git_client.clone())
                .root("https://raw.githubusercontent.com")
                .user("SiegeEngineers")
                .repo("aoc-reference-data")
                .uri("master/data")
                .file(file.clone())
                .build(),
        );

        if let Some(request) = request {
            let response = request.execute().await?;

            match file.ext() {
                FileFormat::Json => match file.name().as_str() {
                    "platforms" => {
                        let mut locked = reference_db.lock().await;
                        locked.platforms =
                            response.json::<Vec<platforms::Platforms>>().await?
                    }
                    "teams" => {
                        let mut locked = reference_db.lock().await;
                        locked.teams =
                            response.json::<Vec<teams::Teams>>().await?
                    }
                    _ => {}
                },
                FileFormat::Yaml => {
                    if let "players" = file.name().as_str() {
                        let mut locked = reference_db.lock().await;
                        locked.players =
                            serde_yaml::from_slice::<Vec<players::Players>>(
                                &response.bytes().await?,
                            )
                            .unwrap()
                    }
                }
                _ => {}
            }
        } else {
            todo!()
        }
    }

    Ok(())
}
