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

#[must_use]
pub fn transparencies(
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health_check().or(matchinfo(client.clone(), ref_data.clone()))
}

/// GET /`health_check`
pub fn health_check(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health_check")
        .and(warp::get())
        .and_then(return_health_check_to_client)
}

/// GET  /matchinfo?id_type=profile_id&id_number=459658
pub fn matchinfo(
    client: reqwest::Client,
    ref_data: Arc<Mutex<RefDataLists>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let client_filter = warp::any().map(move || client.clone());
    let ref_data_filter = warp::any().map(move || ref_data.clone());

    warp::path!("matchinfo")
        .and(warp::get())
        .and(warp::query::<MatchInfoRequest>())
        .and(client_filter)
        .and(ref_data_filter)
        .and_then(return_matchinfo_to_client)
}
