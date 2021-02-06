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

// Internal Configuration
use transparencies_backend_rs::{
    domain::api_handler::client::ApiRequest,
    server::{
        filters,
        models,
    },
    setup::{
        cli::CommandLineSettings,
        configuration::get_configuration,
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
    let _configuration =
        get_configuration().expect("Failed to read configuration.");

    // Calling the command line parsing logic with the argument values
    let cli_args = CommandLineSettings::from_args();

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

    let api = filters::transparencies();

    let routes = api.with(warp::log("transparencies"));

    warp::serve(routes)
        // TODO: Activate after certificates have been received from Let's Encrypt
        // .tls()
        // .cert_path("examples/tls/cert.pem")
        // .key_path("examples/tls/key.rsa")
        .run(([127, 0, 0, 1], 8000))
        .await;
}

#[cfg(test)]
mod tests {
    use transparencies_backend_rs::server::{
        self,
        filters,
    };
    use warp::{
        http::StatusCode,
        test::request,
    };

    #[tokio::test]
    async fn test_health_check_is_reachable() {
        let api = filters::health_check();

        let resp = request()
            .method("GET")
            .path("/health_check")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_matchinfo_is_reachable() {
        let api = filters::matchinfo();

        let resp = request()
            .method("GET")
            .path("/matchinfo?id_type=profile_id&id_number=459658")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
