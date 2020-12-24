//! Core logic of the application
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]


pub mod cli;

use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use stable_eyre::eyre::{
    eyre,
    Report,
    Result,
    WrapErr,
};

/// Entrypoint for the library part of the Executable's main function
pub fn run(config: cli::Args) -> Result<(), Report> {
    debug!("CLI config: {:#?}", config);
    trace!("We are inside the run-function!");

    Ok(())
}