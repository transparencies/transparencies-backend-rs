//! Core logic of the application
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub mod cli;
pub mod configuration;
pub mod domain;
pub mod persistence;
pub mod presentation;
pub mod routes;
pub mod startup;

use futures::{FutureExt, StreamExt};
use log::{debug, error, info, trace, warn};
use stable_eyre::eyre::{eyre, Result, WrapErr};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

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
