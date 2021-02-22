//! Our API endpoints

use std::sync::Arc;

use crate::{
    domain::api_handler::{
        client::ApiClient,
        response::aoc_ref::RefDataLists,
    },
    server::{
        handlers::{
            return_health_check_to_client,
            return_matchinfo_to_client,
        },
        models::MatchInfoRequest,
    },
};

use tokio::sync::Mutex;
use warp::Filter;

/// A general warp-filter that is basically our API
#[must_use]
pub fn transparencies(
    aoe_net_client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health_check().or(matchinfo(aoe_net_client, ref_data))
}

/// GET `/health_check`
/// Our health check to see whether our server is up and running or not
pub fn health_check(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health_check")
        .and(warp::get())
        .and_then(return_health_check_to_client)
}

/// GET  `/matchinfo?id_type=profile_id&id_number=459658`
/// Our matchinfo endpoint
pub fn matchinfo(
    aoe_net_client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Basically a filter that listens on all endpoints which just wraps and
    // forwards our aoe_net_client to make it reusable between requests to
    // our API This is warp specific to give the client as context to our
    // `return_matchinfo_to_client` function
    let aoe_net_client_filter = warp::any().map(move || aoe_net_client.clone());

    // A filter that wraps the `in-memory DB` of aoc_reference_data so we can
    // have it as a context in data processing
    let ref_data_filter = warp::any().map(move || ref_data.clone());

    warp::path!("matchinfo")
        .and(warp::get())
        .and(warp::query::<MatchInfoRequest>())
        .and(aoe_net_client_filter)
        .and(ref_data_filter)
        .and_then(return_matchinfo_to_client)
}
