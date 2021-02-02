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

// Configuration
use transparencies_backend_rs::configuration::get_configuration;

// Startup
use transparencies_backend_rs::startup::run;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    // Install the panic and error report handlers
    stable_eyre::install()?;

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

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

    // Calling the command line parsing logic with the argument values
    let cli_args = transparencies_backend_rs::cli::Args::from_args();

    // If `debug` flag is set, we use a logfile
    if cli_args.debug {
        // Setting up logfile
        let log_setup = LogConfigBuilder::builder()
            .path(&cli_args.log_file_path)
            .size(1 * 100)
            .roll_count(10)
            .level(&cli_args.log_level)
            .output_file()
            .output_console()
            .build();

        simple_log::new(log_setup.clone()).expect("Log setup failed!");
        debug!("Log config: {:?}", &log_setup);
        trace!("Logs were set up.");
    }

    // Setting up any other configuration
    let configuration =
        get_configuration().expect("Failed to read configuration.");

    // Binding address
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    // let response =
    //     transparencies_backend_rs::domain::api_handler::client::
    // get_from_aoe2net().await?;

    // println!("{:#?}", response);

    // Calling run function in lib.rs
    // Handling the error if run returns an error
    match run(listener /* &cli_args */)?.await {
        Err(e) => Err(e).wrap_err("overlay-server experienced a failure!"),
        Ok(k) => Ok(k),
    }
}
