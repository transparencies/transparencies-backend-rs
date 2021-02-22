use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    domain::api_handler::{
        client::{ApiRequest, ApiRequestBuilder, Response},
        response::{
            aoc_ref::{
                platforms::PlatformsList, players::PlayersList,
                teams::TeamsList,
            },
            aoe2net::{
                last_match::PlayerLastMatch, leaderboard::LeaderboardInfo,
                rating_history::RatingHistory,
            },
        },
    },
    server::models::MatchInfoRequest,
};
use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};

use std::{sync::Arc, time::Duration};

use super::api_handler::{
    client::{
        ApiClient, File, FileFormat, GithubFileRequest,
        GithubFileRequestBuilder,
    },
    response::aoc_ref::{platforms, players, teams, RefDataLists},
};

// App-Name as USERAGENT
static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInfo {}

#[derive(Debug, Default, Serialize)]
pub struct MatchDataResponses {
    last_match: PlayerLastMatch,
    leaderboard: LeaderboardInfo,
    rating_history: Vec<RatingHistory>,
    github: RefDataLists,
}

pub async fn process_matchinfo_request(
    par: MatchInfoRequest,
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> Result<MatchDataResponses> {
    debug!(
        "MatchInfoRequest: {:?} with {:?}",
        par.id_type, par.id_number
    );

    let mut responses = MatchDataResponses::default();

    // GET `LastMatch` data
    let last_match_request: Option<ApiRequest> = match &par.id_number {
        Some(id_number) => match &par.id_type {
            Some(id_type) => match id_type.as_str() {
                "steam_id" | "profile_id" => Some(
                    ApiRequestBuilder::default()
                        .client(client.clone())
                        .root("https://aoe2.net/api")
                        .endpoint("player/lastmatch")
                        .query(vec![
                            ("game".to_string(), "aoe2de".to_string()),
                            (id_type.clone(), id_number.clone()),
                        ])
                        .build()
                        .unwrap(),
                ),
                _ => None,
            },
            None => {
                todo!()
            }
        },
        None => {
            todo!()
        }
    };

    // GET `Leaderboard` data
    let leaderboard_request: Option<ApiRequest> = match &par.id_number {
        Some(id_number) => match &par.id_type {
            Some(id_type) => match id_type.as_str() {
                "steam_id" | "profile_id" => Some(
                    ApiRequestBuilder::default()
                        .client(client.clone())
                        .root("https://aoe2.net/api")
                        .endpoint("leaderboard")
                        .query(vec![
                            ("game".to_string(), "aoe2de".to_string()),
                            (id_type.clone(), id_number.clone()),
                            (
                                "leaderboard_id".to_string(),
                                responses
                                    .last_match
                                    .last_match
                                    .leaderboard_id
                                    .to_string(),
                            ),
                        ])
                        .build()
                        .unwrap(),
                ),
                _ => None,
            },
            None => {
                todo!()
            }
        },
        None => {
            todo!()
        }
    };

    // GET `RatingHistory` data
    let rating_history_request: Option<ApiRequest> = match &par.id_number {
        Some(id_number) => match &par.id_type {
            Some(id_type) => match id_type.as_str() {
                "steam_id" | "profile_id" => Some(
                    ApiRequestBuilder::default()
                        .client(client.clone())
                        .root("https://aoe2.net/api")
                        .endpoint("player/ratinghistory")
                        .query(vec![
                            ("game".to_string(), "aoe2de".to_string()),
                            (id_type.clone(), id_number.clone()),
                            (
                                "leaderboard_id".to_string(),
                                responses
                                    .last_match
                                    .last_match
                                    .leaderboard_id
                                    .to_string(),
                            ),
                            ("count".to_string(), "1".to_string()),
                        ])
                        .build()
                        .unwrap(),
                ),
                _ => None,
            },
            None => {
                todo!()
            }
        },
        None => {
            todo!()
        }
    };

    if let Some(request) = last_match_request {
        responses.last_match =
            request.execute::<PlayerLastMatch>().await.unwrap();
    } else {
        todo!()
    }

    if let Some(request) = leaderboard_request {
        responses.leaderboard =
            request.execute::<LeaderboardInfo>().await.unwrap();
    } else {
        todo!()
    }

    if let Some(request) = rating_history_request {
        responses.rating_history =
            request.execute::<Vec<RatingHistory>>().await.unwrap();
    } else {
        todo!()
    }

    // DEBUG: Send Github files to frontend
    responses.github = ref_data.lock().await.clone();

    // let data: MatchInfo;

    Ok(responses)
}

pub async fn process_aoc_ref_data_request(
    reference_db: Arc<Mutex<RefDataLists>>
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
            GithubFileRequestBuilder::default()
                .root("https://raw.githubusercontent.com")
                .user("SiegeEngineers")
                .repo("aoc-reference-data")
                .uri("master/data")
                .file(file.clone())
                .build()
                .unwrap(),
        );

        if let Some(request) = request {
            let response = request.execute().await?;

            match file.ext {
                FileFormat::Json => match file.name.as_str() {
                    "platforms" => {
                        reference_db.lock().await.platforms =
                            response.json::<Vec<platforms::Platforms>>().await?
                        // .into_boxed_slice()
                    }
                    "teams" => {
                        reference_db.lock().await.teams =
                            response.json::<Vec<teams::Teams>>().await?
                        // .into_boxed_slice()
                    }
                    _ => {}
                },
                FileFormat::Yaml => match file.name.as_str() {
                    "players" => {
                        reference_db.lock().await.players =
                            serde_yaml::from_slice::<Vec<players::Players>>(
                                &response.bytes().await?,
                            )
                            .unwrap()
                        // .into_boxed_slice()
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            todo!()
        }
    }

    Ok(())
}

// root: https://raw.githubusercontent.com
// user: SiegeEngineers
// repo: aoc-reference-data
// uri: master/data
// file: File {
//         name: players
//         ext: FileFormat::Yaml
// }

// pub async fn get_from_aoe2net(
//     root: String,
//     endpoint: String,
//     query: Vec<(String, String)>,
// ) -> eyre::Result<Response<PlayerLastMatch>> {
//     let request: ApiRequest = ApiRequestBuilder::default()
//         .root(root)
//         .endpoint(endpoint)
//         .query(query)
//         .build()
//         .unwrap();

//     let response = request.execute::<PlayerLastMatch>().await?;

//     Ok(response)
// }
