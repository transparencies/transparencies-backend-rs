//! Everything regarding startup of the backend

use std::env;

use crate::setup::cli::CommandLineSetting;

use stable_eyre::eyre::{
    Report,
    Result,
};

use crate::setup::telemetry::{
    get_subscriber,
    init_subscriber,
};

/// Set up the logging infrastructure
///
/// # Errors
// TODO
pub fn set_up_logging(
    cli_args: &impl CommandLineSetting
) -> Result<(), Report> {
    // Webserver logging
    if env::var_os("RUST_LOG").is_none() {
        // Show debug logs only when running with `debug` flags
        if cli_args.debug() & (cli_args.log_level() == "debug") {
            env::set_var("RUST_LOG", "debug");
            env::set_var("RUST_BACKTRACE", "1");
        }
        else if cli_args.debug() & (cli_args.log_level() == "trace") {
            env::set_var("RUST_LOG", "trace");
            env::set_var("RUST_BACKTRACE", "1");
        }
        else {
            // Access logs
            env::set_var("RUST_LOG", "info");
        }
    }

    let subscriber =
        get_subscriber("transparencies".into(), cli_args.log_level().into());
    init_subscriber(subscriber);

    Ok(())
}
