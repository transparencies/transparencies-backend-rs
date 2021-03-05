//! Core logic of the application within the library
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: Deny again, when more doc content is in
#![allow(missing_docs)]

use std::collections::HashMap;

pub mod domain;
pub mod server;
pub mod setup;

#[macro_use]
extern crate lazy_static;

lazy_static! {
/// These are our standard values over the whole library part
static ref STANDARD: HashMap<&'static str, &'static str> = {
        let mut std = HashMap::new();
        // Standard language for everything our http clients requests
        std.insert("language", "en");
        // Standard game within the aoe2net universe
        std.insert("game", "aoe2de");

        std
    };

}
