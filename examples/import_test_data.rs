//! Example executable that parses test data and assembles a [`MatchInfoResult`]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]

// Error handling
// #[macro_use]
extern crate log;

extern crate transparencies_backend_rs;

// CLI
use structopt::StructOpt;

// Threads
use std::sync::Arc;

// Internal Configuration
use transparencies_backend_rs::{
    domain::types::{
        requests::ApiClient,
        InMemoryDb,
    },
    setup::{
        cli::CommandLineSettings,
        startup::set_up_logging,
    },
};

use stable_eyre::eyre::{
    Report,
    Result,
};

#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

use tokio::sync::Mutex;
use transparencies_backend_rs::domain::{
    data_processing::process_match_info_request,
    in_memory_db::data_preloading::preload_data,
    types::api::MatchInfoRequest,
};

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
    let cli_args = CommandLineSettings::from_args();

    // If `debug` flag is set, we use a logfile
    if cli_args.debug {
        set_up_logging(&cli_args)?;
    }

    let import_path = "tests/matchinfo-integration/resources";

    let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
    let in_memory_db_clone = in_memory_db.clone();
    let api_clients = ApiClient::default();
    let match_info_request = MatchInfoRequest {
        language: Some("en".to_string()),
        game: Some("aoe2de".to_string()),
        id_type: "profile_id".to_string(),
        id_number: "196240".to_string(),
    };

    preload_data(
        Some(api_clients.github.clone()),
        Some(api_clients.aoe2net.clone()),
        in_memory_db_clone.clone(),
        import_path,
    )
    .await
    .expect("Preloading data failed.");

    let _test_result = process_match_info_request(
        match_info_request,
        api_clients.aoe2net.clone(),
        in_memory_db_clone.clone(),
        import_path,
    )
    .await
    .expect("Matchinfo processing failed.");

    // TODO: We have the result from our offline responses here
    // Now we need to parse the `match_info_result.ron` into
    // [`MatchInfoResult`] type (`parsed_result`) and compare the
    // content with `test_result`.
    // The result of `test_result == parsed_result` is our test return
    // value.

    // result.export_data_to_file(
    //     PathBuf::from_str("tests/integration/resources").unwrap(),
    // );

    Ok(())
}
