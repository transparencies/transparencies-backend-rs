//! Core logic of the application within the library
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: Deny again, when more doc content is in
#![allow(missing_docs)]
// Allowed until https://github.com/rust-lang/rust-clippy/issues/6858 is fixed
#![allow(clippy::default_trait_access)]

use std::time::Duration;

use dashmap::DashMap;

pub mod domain;
pub mod persistence;
pub mod server;
pub mod setup;

#[macro_use]
extern crate lazy_static;

lazy_static! {
/// These are our standard values over the whole library part
pub static ref STANDARD: DashMap<&'static str, &'static str> = {
        let std = DashMap::new();
        // Standard language for everything our http clients requests
        std.insert("language", "en");
        // Standard game within the aoe2net universe
        std.insert("game", "aoe2de");

        std
    };

/// All of the current `language strings` of the AoE2.net API
/// used for preloading the Language files
pub static ref LANGUAGE_STRINGS: [&'static str; 18] = [
    "en", "de", "el", "es", "es-MX", "fr", "hi", "it", "ja", "ko", "ms", "nl",
    "pt", "ru", "tr", "vi", "zh", "zh-TW",
];

/// `Game strings` used for preloading and other request towards the AoE2.net
/// API.
/// Can be used later also for adding `AoE3DE` and/or `AoE4` support
pub static ref GAME_STRINGS: [&'static str; 1] = ["aoe2de"];

/// Our app name as USERAGENT for the clients
pub static ref APP_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Timeout for http-client requests
pub static ref CLIENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
/// Timeout for http-connections
pub static ref CLIENT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);


}
