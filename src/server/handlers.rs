//! API handlers, the ends of each filter chain

use crate::domain::api_handler::{
    client::{
        ApiRequest,
        ApiRequestBuilder,
    },
    response::aoe2net::last_match::PlayerLastMatch,
};

use crate::domain::data_processing::process_matchinfo_request;

use crate::{
    domain::data_processing::MatchDataResponses,
    server::models::MatchInfoRequest,
};

use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn return_health_check_to_client(
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply())
}

// GET / Test: http://127.0.0.1:8000/matchinfo?id_type=profile_id&id_number=459658
pub async fn return_matchinfo_to_client(
    opts: MatchInfoRequest
) -> Result<impl warp::Reply, Infallible> {
    let processed_match_info = process_matchinfo_request(opts).await.unwrap();

    Ok(warp::reply::json(&processed_match_info))
    // Ok(warp::reply())
}
