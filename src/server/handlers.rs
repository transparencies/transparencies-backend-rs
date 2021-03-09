//! API handlers, the ends of each filter chain

use crate::domain::{
    data_processing::process_match_info_request,
    types::{
        api::MatchInfoRequest,
        InMemoryDb,
    },
};

use std::{
    convert::Infallible,
    sync::Arc,
};
use tokio::sync::Mutex;

use url::Url;

/// Small `health_check` function to return 200 on `health_check` endpoint
///
/// # Errors
// TODO
pub async fn return_health_check_to_client(
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply())
}

/// Handler function to return data from the `match_info` processing serialized
/// as JSON to `/matchinfo` endpoint
///
/// GET Endpoint
/// Possible test url: <http://127.0.0.1:8000/matchinfo?id_type=profile_id&id_number=196240&game=aoe2de>
///
/// - `opts`: options struct that contains the parameters that the client gave
///   us
/// - `aoe_net_client`: Our reusable aoe.net Client
/// - `ref_data`: We take an `Arc<Mutex<T>>` as parameter which is mimicking our
///   in-memory DB for the files from Github
///
/// # Errors
// TODO
/// # Panics
// TODO
#[allow(clippy::let_unit_value)]
pub async fn return_matchinfo_to_client(
    opts: MatchInfoRequest,
    aoe_net_client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<impl warp::Reply, Infallible> {
    // API root for aoe2net
    let root = Url::parse("https://aoe2.net/api").unwrap();

    let processed_match_info = process_match_info_request(
        opts,
        aoe_net_client.clone(),
        root,
        in_memory_db.clone(),
        None,
    )
    .await
    .expect("Matchinfo processing failed.");

    Ok(warp::reply::json(&processed_match_info))
}
