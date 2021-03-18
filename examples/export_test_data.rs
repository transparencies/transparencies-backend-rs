//! Example executable that pulls in test data and saves them for our
//! integration tests

#![deny(clippy::all)]
#![deny(clippy::pedantic)]

extern crate transparencies_backend_rs;

// CLI
use structopt::StructOpt;

// Threads
use std::sync::Arc;

// Internal Configuration
use transparencies_backend_rs::{
    domain::{
        api_handler::client::A2NClient,
        types::InMemoryDb,
    },
    setup::{
        cli::ExportCommandLineSettings,
        startup::set_up_logging,
    },
    APP_USER_AGENT,
    CLIENT_CONNECTION_TIMEOUT,
    CLIENT_REQUEST_TIMEOUT,
};

use stable_eyre::eyre::{
    Report,
    Result,
};

#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

use std::path::PathBuf;
use tokio::sync::Mutex;
use transparencies_backend_rs::{
    domain::{
        data_processing::build_result,
        types::api::MatchInfoRequest,
    },
    persistence::in_memory_db::data_preloading::preload_data,
};

use url::Url;

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

    // Calling the command line parsing logic with the argument values
    let cli_args = ExportCommandLineSettings::from_args();

    // If `debug` flag is set, we use a logfile
    if cli_args.debug {
        set_up_logging(&cli_args)?;
    }
    let current_dir = std::env::current_dir().unwrap();

    let export_path: Option<PathBuf> = Some(
        [
            &format!("{}", current_dir.display()),
            &format!("{}", &cli_args.test_case_export_path),
        ]
        .iter()
        .collect(),
    );
    let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
    let in_memory_db_clone = in_memory_db.clone();
    // let api_clients = ApiClient::default();
    let match_info_request =
        MatchInfoRequest::with_folder(export_path.clone().unwrap());

    // match_info_request
    //     .export_data_to_file(PathBuf::from_str(export_path.unwrap()).
    // unwrap());

    let client = reqwest::Client::builder()
        .user_agent(*APP_USER_AGENT)
        .timeout(*CLIENT_REQUEST_TIMEOUT)
        .connect_timeout(*CLIENT_CONNECTION_TIMEOUT)
        .use_rustls_tls()
        .https_only(true)
        .build()
        .unwrap();

    let a2n_client = A2NClient::with_client(client.clone());

    let github_root = Url::parse("https://raw.githubusercontent.com")?;
    let aoe2_net_root = Url::parse("https://aoe2.net/api")?;

    preload_data(
        Some(client.clone()),
        Some(client.clone()),
        in_memory_db_clone.clone(),
        github_root,
        aoe2_net_root.clone(),
        export_path.clone(),
        false,
    )
    .await
    .expect("Preloading data failed.");

    let result = build_result(
        match_info_request,
        a2n_client,
        aoe2_net_root,
        in_memory_db_clone.clone(),
        export_path.clone(),
    )
    .await;

    result.export_to_file(export_path.unwrap());

    Ok(())
}
