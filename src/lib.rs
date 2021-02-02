//! Core logic of the application
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub mod cli;
pub mod domain;
pub mod persistence;
pub mod presentation;

use futures::{FutureExt, StreamExt};
use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Result, WrapErr};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Entrypoint for the library part of the Executable's main function
pub fn run(/*config: &cli::Args*/) -> eyre::Result<Server, std::io::Error> {
    // debug!("CLI config: {:#?}", config);
    trace!("We are inside the run-function!");

    // needed endpoints
    // rating/?steam_id=<Steam-ID>
    // rating/?profile_id=<ageofempires.com-Profile-ID>

    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}

// Notes

// WEBSOCKET
// let routes = warp::path("rating")
//     // The `ws()` filter will prepare the Websocket handshake.
//     .and(warp::ws())
//     .map(|ws: warp::ws::Ws| {
//         // And then our closure will be called when it completes...
//         ws.on_upgrade(|websocket| {
//             // Just echo all messages back...
//             let (tx, rx) = websocket.split();
//             rx.forward(tx).map(|result| {
//                 if let Err(e) = result {
//                     eprintln!("websocket error: {:?}", e);
//                 }
//             })
//         })
//     });

// let opt_query = warp::query::<MyObject>()
//     .map(Some)
//     .or_else(|_| async { Ok::<(Option<MyObject>,), std::convert::Infallible>((None,)) });

// // get /rating?steam_id=<Steam-ID>
// let rating = warp::get()
//     .and(warp::path("rating"))
//     .and(opt_query)
//     .map(|p: Option<MyObject>| match p {
//         Some(obj) => Response::builder().body(format!(
//             "steam_id = {}, profile_id = {}",
//             obj.steam_id, obj.profile_id
//         )),
//         None => Response::builder()
//             .status(StatusCode::BAD_REQUEST)
//             .body(String::from("Failed to decode query param.")),
//     });

// ---
// We have everything we need to go to API-Handler
// to generate other API requests for a request from frontend
// ---

// let response = domain::api_handler::client::get_from_aoe2net().await?;

// println!("{:#?}", response);

// API_HANDLER
// Get & deserialize yaml into struct
// https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/players.yaml

// Get and deserialize match data into struct
// https://github.com/seanmonstar/reqwest/blob/master/examples/json_typed.rs

// warp::serve(rating).run(([127, 0, 0, 1], 3030)).await;

// Ok(())
