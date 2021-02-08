use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    domain::api_handler::{
        client::{
            ApiRequest,
            ApiRequestBuilder,
            Response,
        },
        response::{
            aoc_ref::{
                platforms::PlatformsList,
                players::PlayersList,
                teams::TeamsList,
            },
            aoe2net::{
                last_match::PlayerLastMatch,
                leaderboard::LeaderboardInfo,
                rating_history::RatingHistory,
            },
        },
    },
    server::models::MatchInfoRequest,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInfo {}

#[derive(Debug, Default, Serialize)]
pub struct MatchDataResponses {
    last_match: PlayerLastMatch,
    leaderboard: LeaderboardInfo,
    rating_history: Vec<RatingHistory>,
}

pub struct AocDataLists {
    platforms: PlatformsList,
    players: PlayersList,
    teams: TeamsList,
}

pub async fn process_matchinfo_request(
    par: MatchInfoRequest
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
    }
    else {
        todo!()
    }

    if let Some(request) = leaderboard_request {
        responses.leaderboard =
            request.execute::<LeaderboardInfo>().await.unwrap();
    }
    else {
        todo!()
    }

    if let Some(request) = rating_history_request {
        responses.rating_history =
            request.execute::<Vec<RatingHistory>>().await.unwrap();
    }
    else {
        todo!()
    }

    // let data: MatchInfo;

    Ok(responses)
}

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
