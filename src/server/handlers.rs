//! API handlers, the ends of each filter chain

use crate::domain::api_handler::{
    client::{ApiClient, ApiRequest, ApiRequestBuilder},
    response::{aoc_ref::RefDataLists, aoe2net::last_match::PlayerLastMatch},
};

use crate::domain::data_processing::process_match_info_request;

use crate::{
    domain::data_processing::MatchDataResponses,
    server::models::MatchInfoRequest,
};

use log::{debug, error, info, trace, warn};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::http::StatusCode;

/// Small `health_check` function to return 200 on `health_check` endpoint
pub async fn return_health_check_to_client(
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply())
}

/// Handler function to return data from the match_info processing serialized as JSON to `/matchinfo` endpoint
///
/// GET Endpoint
/// Possible test url: http://127.0.0.1:8000/matchinfo?id_type=profile_id&id_number=459658
///
/// aoe_net_client: Our reusable aoe.net Client
/// ref_data: We take an `Arc<Mutex<T>>` as parameter which is mimicking our in-memory DB for the files from Github
pub async fn return_matchinfo_to_client(
    opts: MatchInfoRequest,
    aoe_net_client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> Result<impl warp::Reply, Infallible> {
    let processed_match_info = process_match_info_request(
        opts,
        aoe_net_client.clone(),
        ref_data.clone(),
    )
    .await
    .unwrap();

    Ok(warp::reply::json(&processed_match_info))
    // Ok(warp::reply())
}
