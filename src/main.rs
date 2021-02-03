//! Executable for managing aoe-reference-data files.
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

// Error handling
#[macro_use]
extern crate log;
use human_panic::setup_panic;
use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use simple_log::LogConfigBuilder;
use stable_eyre::eyre::{
    eyre,
    Result,
    WrapErr,
};
use std::{
    env,
    process,
};

// Binding
use std::net::TcpListener;

// CLI
use structopt::StructOpt;

// Internal Configuration
use transparencies_backend_rs::setup::{
    cli::CommandLineSettings,
    configuration::get_configuration,
    startup::run_server,
};

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    // Install the panic and error report handlers
    stable_eyre::install()?;

    // TODO: Webserver logging
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!(Metadata {
            name: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: "the transparencies authors".into(),
            homepage: "https://github.com/transparencies/transparencies-backend-rs/issues".into(),
        });
    }

    // Setting up configuration
    let mut configuration =
        get_configuration().expect("Failed to read configuration.");

    // Calling the command line parsing logic with the argument values
    configuration.cli = CommandLineSettings::from_args();

    // If `debug` flag is set, we use a logfile
    if configuration.cli.debug {
        // Setting up logfile
        let log_setup = LogConfigBuilder::builder()
            .path(&configuration.cli.log_file_path)
            .size(1 * 100)
            .roll_count(10)
            .level(&configuration.cli.log_level)
            .output_file()
            .output_console()
            .build();

        simple_log::new(log_setup.clone()).expect("Log setup failed!");
        debug!("Log config: {:?}", &log_setup);
        trace!("Logs were set up.");
    }

    // Binding address
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    // Calling run function in lib.rs
    // Handling the error if run returns an error
    match run_server(listener, &configuration.cli)?.await {
        Err(e) => Err(e).wrap_err("overlay-server experienced a failure!"),
        Ok(k) => Ok(k),
    }
}
