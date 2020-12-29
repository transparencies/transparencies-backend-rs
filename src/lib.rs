//! Core logic of the application
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub mod api_responses;
pub mod cli;
pub mod client;

use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Result, WrapErr};

/// Entrypoint for the library part of the Executable's main function
pub async fn run(config: &cli::Args) -> eyre::Result<()> {
    debug!("CLI config: {:#?}", config);
    trace!("We are inside the run-function!");

    let response = client::get_from_aoe2net().await?;

    println!("{:#?}", response);

    // Server-side (actix-web)

    // API_HANDLER
    // Get & deserialize yaml into struct
    // https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/players.yaml

    // Get and deserialize match data into struct
    // https://github.com/seanmonstar/reqwest/blob/master/examples/json_typed.rs

    // WEB_FRAMEWORK
    // Set up routes for frontend
    // websocket
    // https://github.com/actix/examples/blob/master/websocket/src/main.rs

    Ok(())
}
