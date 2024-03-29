//! Backend for dynamic stream overlays
//! Executable part
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

extern crate transparencies_backend_rs;

// Threads
use std::{
    net::IpAddr,
    sync::Arc,
};

#[cfg(not(debug_assertions))]
use human_panic::setup_panic;
use stable_eyre::eyre::{
    Report,
    Result,
};
// CLI
use structopt::StructOpt;
use tokio::sync::Mutex;
// Internal Configuration
use transparencies_backend_rs::{
    domain::{
        api_handler::client::A2NClient,
        types::InMemoryDb,
    },
    persistence::in_memory_db::data_preloading::get_static_data_inside_thread,
    server::filters,
    setup::{
        cli::CommandLineSettings,
        configuration::get_configuration,
        startup::set_up_logging,
    },
    APP_USER_AGENT,
    CLIENT_CONNECTION_TIMEOUT,
    CLIENT_REQUEST_TIMEOUT,
};
use url::Url;
use warp::Filter;

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

    let client =
        reqwest::Client::builder().user_agent(*APP_USER_AGENT)
                                  .timeout(*CLIENT_REQUEST_TIMEOUT)
                                  .connect_timeout(*CLIENT_CONNECTION_TIMEOUT)
                                  .use_rustls_tls()
                                  .https_only(true)
                                  .build()
                                  .unwrap();

    let github_root = Url::parse("https://raw.githubusercontent.com")?;
    let aoe2_net_root = Url::parse("https://aoe2.net/api")?;

    get_static_data_inside_thread(in_memory_db_clone,
                                  github_root,
                                  aoe2_net_root).await;

    let a2n_client = A2NClient::with_client(client);

    let api = filters::transparencies(a2n_client, in_memory_db.clone());

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
