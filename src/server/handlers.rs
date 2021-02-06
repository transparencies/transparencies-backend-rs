//! API handlers, the ends of each filter chain

use crate::domain::api_handler::{
    client::{
        ApiRequest,
        ApiRequestBuilder,
    },
    response::aoe2net::last_match::PlayerLastMatch,
};

use crate::server::models::MatchInfoRequest;

use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn return_health_check() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply())
}

pub async fn return_matchinfo(
    opts: MatchInfoRequest
) -> Result<impl warp::Reply, Infallible> {
    debug!(
        "MatchInfoRequest: {:?} with {:?}",
        opts.id_type, opts.id_number
    );

    let build_request: Option<ApiRequest> = match opts.id_number {
        Some(id_number) => match opts.id_type {
            Some(id_type) => match id_type.as_str() {
                "steam_id" => {
                    // MatchInfoRequestType::SteamId((id_type,
                    // id_number));

                    Some(
                        ApiRequestBuilder::default()
                            .root("https://aoe2.net/api/")
                            .endpoint("player/lastmatch")
                            .query(vec![
                                ("game".to_string(), "aoe2de".to_string()),
                                (id_type, id_number),
                            ])
                            .build()
                            .unwrap(),
                    )
                }
                "profile_id" => {
                    // MatchInfoRequestType::AoeNetProfile((
                    //     id_type, id_number,
                    // ));

                    Some(
                        ApiRequestBuilder::default()
                            .root("https://aoe2.net/api/")
                            .endpoint("player/lastmatch")
                            .query(vec![
                                ("game".to_string(), "aoe2de".to_string()),
                                (id_type, id_number),
                            ])
                            .build()
                            .unwrap(),
                    )
                }
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

    let api_response;

    if let Some(request) = build_request {
        api_response = request.execute::<PlayerLastMatch>().await.unwrap();
        Ok(warp::reply::json(&api_response.response))
    }
    else {
        todo!()
    }
}
