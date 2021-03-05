//! Everything regarding startup of the backend

use std::env;

use simple_log::LogConfigBuilder;
use tracing::{
    debug,
    trace,
};
use tracing_subscriber::{
    prelude::*,
    Registry,
};
use tracing_tree::HierarchicalLayer;

use super::cli::CommandLineSettings;

use stable_eyre::eyre::{
    Report,
    Result,
};

/// Set up the logging infrastructure
pub fn set_up_logging(cli_args: &CommandLineSettings) -> Result<(), Report> {
    // Webserver logging
    if env::var_os("RUST_LOG").is_none() {
        // Show debug logs only when running with `debug` flags
        if cli_args.debug {
            env::set_var("RUST_LOG", "debug");
            env::set_var("RUST_BACKTRACE", "1");
        }
        else {
            // Access logs
            env::set_var("RUST_LOG", "info");
        }
    }

    // install global collector configured based on RUST_LOG env var.
    let subscriber = Registry::default()
        .with(HierarchicalLayer::new(2).with_thread_ids(true));
    tracing::subscriber::set_global_default(subscriber)?;

    // Setting up logfile
    let log_setup = LogConfigBuilder::builder()
        .path(&cli_args.log_file_path)
        .size(100)
        .roll_count(10)
        .level(&cli_args.log_level)
        .output_file()
        .output_console()
        .build();

    simple_log::new(log_setup.clone()).expect("Log setup failed!");
    debug!("Log config: {:?}", &log_setup);
    trace!("Logs were set up.");

    Ok(())
}
