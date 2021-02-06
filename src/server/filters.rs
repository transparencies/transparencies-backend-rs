use crate::server::models::MatchInfoRequest;

use super::{
    handlers,
    models::{Db, ListOptions, Todo},
};
use warp::Filter;

pub fn transparencies(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health_check().or(matchinfo())
}

/// GET /`health_check`
pub fn health_check(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health_check")
        .and(warp::get())
        .and_then(handlers::return_health_check)
}

/// GET /matchinfo?idtype=steamid&idnumber=12318931981421
pub fn matchinfo(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("matchinfo")
        .and(warp::get())
        .and(warp::query::<MatchInfoRequest>())
        .and_then(handlers::return_matchinfo)
}
