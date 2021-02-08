use crate::server::{
    handlers::{
        return_health_check_to_client,
        return_matchinfo_to_client,
    },
    models::MatchInfoRequest,
};

use warp::Filter;

#[must_use]
pub fn transparencies(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health_check().or(matchinfo())
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
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("matchinfo")
        .and(warp::get())
        .and(warp::query::<MatchInfoRequest>())
        .and_then(return_matchinfo_to_client)
}
