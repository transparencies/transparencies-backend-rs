//! Executable for managing aoe-reference-data files.
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(clippy::too_many_lines)]

// Error handling
#[macro_use]
extern crate log;

extern crate transparencies_backend_rs;

use eyre::Error;
use human_panic::setup_panic;
use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use simple_log::LogConfigBuilder;
use std::{
    env,
    process,
};
use warp::{
    http::StatusCode,
    Filter,
};

// CLI
use structopt::StructOpt;

// Threads
use std::sync::Arc;

use tokio::{
    io::AsyncReadExt,
    sync::Mutex,
    time::{
        self,
        Duration,
    },
};

use std::{
    convert::Infallible,
    net::IpAddr,
};

// Internal Configuration
use transparencies_backend_rs::{
    domain::{
        data_processing::get_static_data_inside_thread,
        types::{
            aoc_ref::RefDataLists,
            requests::{
                ApiClient,
                ApiRequest,
            },
        },
    },
    server::filters,
    setup::{
        cli::CommandLineSettings,
        configuration::get_configuration,
        startup::set_up_logging,
    },
};

#[tokio::main]
async fn main() {
    // Install the panic and error report handlers
    // TODO: Temporary disabled due to return value
    // stable_eyre::install();

    // Webserver logging
    if env::var_os("RUST_LOG").is_none() {
        // TODO Deactivate Debug logs
        env::set_var("RUST_LOG", "transparencies=debug");
        // Access logs
        // env::set_var("RUST_LOG", "transparencies=info");
    }

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
    let configuration =
        get_configuration().expect("Failed to read configuration.");

    // Calling the command line parsing logic with the argument values
    let cli_args = CommandLineSettings::from_args();

    // If `debug` flag is set, we use a logfile
    if cli_args.debug {
        set_up_logging(&cli_args);
    }

    let aoc_reference_data = Arc::new(Mutex::new(RefDataLists::new()));
    let aoc_reference_data_clone = aoc_reference_data.clone();

    let api_clients = ApiClient::default();
    let git_client_clone = api_clients.github.clone();

    get_static_data_inside_thread(git_client_clone, aoc_reference_data_clone);

    let api = filters::transparencies(
        api_clients.aoe2net.clone(),
        aoc_reference_data.clone(),
    );

    let routes = api.with(warp::log("transparencies"));

    let ip: IpAddr = configuration.application.host.parse().unwrap();

    warp::serve(routes)
        // TODO: Activate after certificates have been received from Let's Encrypt
        // .tls()
        // .cert_path("examples/tls/cert.pem")
        // .key_path("examples/tls/key.rsa")
        .run((ip, configuration.application.port))
        .await;
}
