pub mod processor;
use processor::MatchInfoProcessor;

use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::domain::{
    api_handler::client::{
        ApiClient,
        ApiRequest,
        File,
        FileFormat,
        GithubFileRequest,
        Response,
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
            match_info::*,
            MatchInfoRequest,
            MatchInfoResult,
        },
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
    eyre,
    Report,
    Result,
    WrapErr,
};

use std::{
    sync::Arc,
    time::Duration,
};

use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize)]
pub struct MatchDataResponses {
    aoe2net: HashMap<String, serde_json::Value>,
    github: RefDataLists,
}

pub async fn process_match_info_request(
    par: MatchInfoRequest,
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
    // ) -> Result<MatchInfoResult> {
) -> Result<MatchInfoResult> {
    debug!(
        "MatchInfoRequest: {:?} with {:?}",
        par.id_type, par.id_number
    );

    let responses = get_match_data_responses(par, client, ref_data).await;
    let _processor = MatchInfoProcessor::load_responses(responses);

    // Fill Structs with data by passing in `responses` into the
    // `MatchDataProcessor` that returns a `MatchInfoResult`

    // Read in Teams
    // Read Players into Teams
    // Read Ratings into Players
    // Assemble Information to MatchInfo
    // Wrap MatchInfo with Erros into MatchInfoResult

    let result: MatchInfoResult = MatchInfoProcessor::create_result();

    Ok(result)
}

async fn get_match_data_responses(
    par: MatchInfoRequest,
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> MatchDataResponses {
    let mut api_requests: Vec<(String, ApiRequest)> = Vec::with_capacity(5);
    let mut responses = MatchDataResponses::default();

    // GET `PlayerLastMatch` data
    let last_match_request = ApiRequest::builder()
        .client(client.clone())
        .root("https://aoe2.net/api")
        .endpoint("player/lastmatch")
        .query(vec![
            ("game".to_string(), "aoe2de".to_string()),
            (par.id_type.clone(), par.id_number.clone()),
        ])
        .build();

    responses.aoe2net.insert(
        "player_last_match".to_string(),
        last_match_request.execute().await.unwrap(),
    );

    // Get `leaderboard_id` for future requests
    let leaderboard_id =
        MatchInfoProcessor::get_leaderboard_id_for_request(&responses)
            .to_string();

    // GET `Leaderboard` data
    api_requests.push((
        "leaderboard".to_string(),
        ApiRequest::builder()
            .client(client.clone())
            .root("https://aoe2.net/api")
            .endpoint("leaderboard")
            .query(vec![
                ("game".to_string(), "aoe2de".to_string()),
                (par.id_type.clone(), par.id_number.clone()),
                ("leaderboard_id".to_string(), leaderboard_id.clone()),
            ])
            .build(),
    ));

    // GET `RatingHistory` data
    api_requests.push((
        "rating_history".to_string(),
        ApiRequest::builder()
            .client(client.clone())
            .root("https://aoe2.net/api")
            .endpoint("player/ratinghistory")
            .query(vec![
                ("game".to_string(), "aoe2de".to_string()),
                (par.id_type.clone(), par.id_number.clone()),
                ("leaderboard_id".to_string(), leaderboard_id),
                ("count".to_string(), "1".to_string()),
            ])
            .build(),
    ));

    for (response_name, req) in &api_requests {
        responses
            .aoe2net
            .insert(response_name.to_string(), req.execute().await.unwrap());
    }

    // Include github response
    responses.github = ref_data.lock().await.clone();

    responses
}

pub async fn process_aoc_ref_data_request(
    git_client: reqwest::Client,
    reference_db: Arc<Mutex<RefDataLists>>,
) -> Result<()> {
    let files: Vec<File> = vec![
        File {
            name: "players".to_string(),
            ext: FileFormat::Yaml,
        },
        File {
            name: "platforms".to_string(),
            ext: FileFormat::Json,
        },
        File {
            name: "teams".to_string(),
            ext: FileFormat::Json,
        },
    ];

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

            match file.ext {
                FileFormat::Json => match file.name.as_str() {
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
                    if let "players" = file.name.as_str() {
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
        }
        else {
            todo!()
        }
    }

    Ok(())
}
