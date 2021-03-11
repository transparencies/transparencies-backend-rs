//! Backend for dynamic stream overlays
//! Executable part
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

// Error handling
// #[macro_use]
extern crate log;

extern crate transparencies_backend_rs;

use warp::Filter;

// CLI
use structopt::StructOpt;

// Threads
use std::sync::Arc;

use tokio::sync::Mutex;

use std::net::IpAddr;

// Internal Configuration
use transparencies_backend_rs::{
    domain::types::{
        requests::ApiClient,
        InMemoryDb,
    },
    persistence::in_memory_db::data_preloading::get_static_data_inside_thread,
    server::filters,
    setup::{
        cli::CommandLineSettings,
        configuration::get_configuration,
        startup::set_up_logging,
    },
};

use stable_eyre::eyre::{
    Report,
    Result,
};

use url::Url;

#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[tokio::main]
async fn main() -> Result<(), Report> {
    // Install the panic and error report handlers
    stable_eyre::install()?;

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
    let configuration = get_configuration()?;

    // Calling the command line parsing logic with the argument values
    let cli_args = CommandLineSettings::from_args();

    // If `debug` flag is set, we use a logfile
    if cli_args.debug {
        set_up_logging(&cli_args)?;
    }

    let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
    let in_memory_db_clone = in_memory_db.clone();

    let api_clients = ApiClient::default();
    let git_client_clone = api_clients.github.clone();
    let aoe2net_client_clone = api_clients.aoe2net.clone();

    let github_root = Url::parse("https://raw.githubusercontent.com")?;
    let aoe2_net_root = Url::parse("https://aoe2.net/api")?;

    get_static_data_inside_thread(
        git_client_clone,
        aoe2net_client_clone,
        in_memory_db_clone,
        github_root,
        aoe2_net_root,
    )
    .await;

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

    Ok(())
}
