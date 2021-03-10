//! Core logic of the application within the library
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: Deny again, when more doc content is in
#![allow(missing_docs)]
// Allowed until https://github.com/rust-lang/rust-clippy/issues/6858 is fixed
#![allow(clippy::default_trait_access)]

use std::collections::HashMap;

pub mod domain;
pub mod persistence;
pub mod server;
pub mod setup;

#[macro_use]
extern crate lazy_static;

lazy_static! {
/// These are our standard values over the whole library part
pub static ref STANDARD: HashMap<&'static str, &'static str> = {
        let mut std = HashMap::new();
        // Standard language for everything our http clients requests
        std.insert("language", "en");
        // Standard game within the aoe2net universe
        std.insert("game", "aoe2de");

        std
    };

}
