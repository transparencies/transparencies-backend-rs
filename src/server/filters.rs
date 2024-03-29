//! API endpoints of the backend

use std::sync::Arc;

use tokio::sync::Mutex;
use warp::Filter;

use crate::{
    domain::{
        api_handler::client::A2NClient,
        types::{
            api::MatchInfoRequest,
            InMemoryDb,
        },
    },
    server::handlers::{
        return_health_check_to_client,
        return_matchinfo_to_client,
    },
};

/// A general warp-filter that is basically our API
#[must_use]
pub fn transparencies(
    aoe_net_client: A2NClient<'static, reqwest::Client>,
    in_memory_db: Arc<Mutex<InMemoryDb>>)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    health_check().or(matchinfo(aoe_net_client, in_memory_db))
}

/// GET `/health_check`
/// Our health check to see whether our server is up and running or not
pub fn health_check(
    )
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("health_check").and(warp::get())
                               .and_then(return_health_check_to_client)
}

/// GET  `/matchinfo?id_type=profile_id&id_number=459658`
/// Our matchinfo endpoint
pub fn matchinfo(
    aoe_net_client: A2NClient<'static, reqwest::Client>,
    in_memory_db: Arc<Mutex<InMemoryDb>>)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    // Basically a filter that listens on all endpoints which just wraps and
    // forwards our aoe_net_client to make it reusable between requests to
    // our API This is warp specific to give the client as context to our
    // `return_matchinfo_to_client` function
    let aoe_net_client_filter = warp::any().map(move || aoe_net_client.clone());

    // A filter that wraps the `in-memory DB` of aoc_reference_data so we can
    // have it as a context in data processing
    let ref_data_filter = warp::any().map(move || in_memory_db.clone());

    warp::path!("matchinfo").and(warp::get())
                            .and(warp::query::<MatchInfoRequest>())
                            .and(aoe_net_client_filter)
                            .and(ref_data_filter)
                            .and_then(return_matchinfo_to_client)
}
