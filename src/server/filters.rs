use std::sync::Arc;

use crate::{
    domain::api_handler::client::ApiClient,
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
    client: reqwest::Client
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health_check().or(matchinfo(client.clone()))
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
    client: reqwest::Client
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let client_filter = warp::any().map(move || client.clone());

    warp::path!("matchinfo")
        .and(warp::get())
        .and(warp::query::<MatchInfoRequest>())
        .and(client_filter)
        .and_then(return_matchinfo_to_client)
}
