//! Core client logic of the application
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Report, Result, WrapErr};

use ::serde::Deserialize;
use std::time::Duration;

use response_datastructures::aoe2net::rating_history::RatingHistory;

// App-Name as USERAGENT
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub async fn get_from_aoe2net() -> eyre::Result<Vec<RatingHistory>> {
    // Duration for timeouts
    let request_timeout: Duration = Duration::new(5, 0);
    let connection_timeout: Duration = Duration::new(5, 0);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .timeout(request_timeout)
        .connect_timeout(connection_timeout)
        .use_rustls_tls()
        .https_only(true)
        .build()?;

    let response = client
        .get("https://aoe2.net/api/player/ratinghistory")
        .query(&[
            ("game", "aoe2de"),
            ("leaderboard_id", "3"),
            ("steam_id", "76561199003184910"),
            ("count", "5"),
        ])
        .send()
        .await?
        .json::<Vec<RatingHistory>>()
        .await?;

    Ok(response)
}
