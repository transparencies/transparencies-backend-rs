//! Backend for dynamic stream overlays
//! Executable part
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

// Error handling
#[macro_use]
extern crate log;

extern crate transparencies_backend_rs;

use std::env;
use warp::Filter;

// CLI
use structopt::StructOpt;

// Threads
use std::sync::Arc;

use tokio::sync::Mutex;

use std::net::IpAddr;

// Internal Configuration
use transparencies_backend_rs::{
    domain::{
        data_processing::get_static_data_inside_thread,
        types::{
            requests::ApiClient,
            InMemoryDb,
        },
    },
    server::filters,
    setup::{
        cli::CommandLineSettings,
        configuration::get_configuration,
        startup::set_up_logging,
    },
};

use tracing_subscriber::{
    prelude::*,
    Registry,
};
use tracing_tree::HierarchicalLayer;

#[tokio::main]
async fn main() {
    // Install the panic and error report handlers
    // TODO: Temporary disabled due to return value
    // stable_eyre::install();

    // Webserver logging
    if env::var_os("RUST_LOG").is_none() {
        // TODO Deactivate Debug logs
        env::set_var("RUST_LOG", "transparencies=debug");
        env::set_var("RUST_BACKTRACE", "1");
        // Access logs
        // env::set_var("RUST_LOG", "transparencies=info");
    }
    // install global collector configured based on RUST_LOG env var.
    //   tracing_subscriber::fmt::init();
    let subscriber = Registry::default().with(HierarchicalLayer::new(2));
    tracing::subscriber::set_global_default(subscriber).unwrap();

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

    // TODO Replace in-memory DB here with a struct holding different fields
    let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
    let in_memory_db_clone = in_memory_db.clone();

    let api_clients = ApiClient::default();
    let git_client_clone = api_clients.github.clone();
    let aoe2net_client_clone = api_clients.aoe2net.clone();

    get_static_data_inside_thread(
        git_client_clone,
        aoe2net_client_clone,
        in_memory_db_clone,
    );

    let api = filters::transparencies(
        api_clients.aoe2net.clone(),
        in_memory_db.clone(),
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
