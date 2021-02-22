//! API handlers, the ends of each filter chain

use crate::domain::api_handler::{
    client::{
        ApiClient,
        ApiRequest,
        ApiRequestBuilder,
    },
    response::{
        aoc_ref::RefDataLists,
        aoe2net::last_match::PlayerLastMatch,
    },
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
use std::{
    convert::Infallible,
    sync::Arc,
};
use tokio::sync::Mutex;
use warp::http::StatusCode;

pub async fn return_health_check_to_client(
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply())
}

// GET / Test: http://127.0.0.1:8000/matchinfo?id_type=profile_id&id_number=459658
pub async fn return_matchinfo_to_client(
    opts: MatchInfoRequest,
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> Result<impl warp::Reply, Infallible> {
    let processed_match_info =
        process_matchinfo_request(opts, client.clone(), ref_data.clone())
            .await
            .unwrap();

    Ok(warp::reply::json(&processed_match_info))
    // Ok(warp::reply())
}
