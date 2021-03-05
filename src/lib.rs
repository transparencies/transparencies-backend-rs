//! Core logic of the application within the library
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: Deny again, when more doc content is in
#![allow(missing_docs)]

pub mod domain;
pub mod server;
pub mod setup;

/// Standard language for everything our http clients requests
pub(crate) static STANDARD_LANGUAGE: &str = "en";

/// Standard game within the aoe2net universe
pub(crate) static STANDARD_GAME: &str = "aoe2de";
