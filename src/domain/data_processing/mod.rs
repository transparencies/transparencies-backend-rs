pub mod error;
mod match_data_responder;
pub mod match_info_processor;
use match_info_processor::MatchInfoProcessor;
pub mod reference_data_handler;

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
use log::{
    debug,
    error,
    info,
    trace,
    warn,
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

use self::reference_data_handler::load_aoc_ref_data;

/// Download static files continously every 10 minutes inside a thread
pub fn get_static_data_inside_thread(
    git_client_clone: reqwest::Client,
    _aoe2net_client_clone: reqwest::Client,
    aoc_reference_data_clone: Arc<Mutex<RefDataLists>>,
) {
    tokio::spawn(async move {
        loop {
            load_aoc_ref_data(
                git_client_clone.clone(),
                aoc_reference_data_clone.clone(),
            )
            .await
            .expect("Unable to load files from Github");

            // TODO
            // Call another function here that pulls in data from various standard languages
            // or fire directly all possible requests, because it's just once every 10 minutes
            // TODO
            // Create enum for all possible languages for later usage or also for requests
            // https://aoe2.net/api/strings?game=aoe2de&language=en

            // Enum content
            // Language (en, de, el, es, es-MX, fr, hi, it, ja, ko, ms, nl, pt, ru, tr, vi, zh, zh-TW)
            // + Game (Age of Empires 2:HD=aoe2hd, Age of Empires 2:Definitive Edition=aoe2de) later for aoe3, aoe4
            

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
) -> Result<MatchInfoResult, ProcessingError> {
    debug!(
        "MatchInfoRequest: {:?} with {:?}",
        par.id_type, par.id_number
    );

    let responses =
        MatchDataResponses::new_with_match_data(par, client, ref_data).await?;

    // Debugging
    responses.export_data_to_file();

    let result = MatchInfoProcessor::new_with_response(responses)
        .process()
        .assemble()?;

    Ok(result)
}
