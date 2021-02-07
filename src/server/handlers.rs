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

// GET / Test: http://127.0.0.1:8000/matchinfo?id_type=profile_id&id_number=459658
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
                "steam_id" | "profile_id" => Some(
                    ApiRequestBuilder::default()
                        .root("https://aoe2.net/api")
                        .endpoint("player/lastmatch")
                        .query(vec![
                            ("game".to_string(), "aoe2de".to_string()),
                            (id_type, id_number),
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

    if let Some(request) = build_request {
        let api_response = request.execute::<PlayerLastMatch>().await.unwrap();
        Ok(warp::reply::json(&api_response.response))
    }
    else {
        todo!()
    }
}
