use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use simple_log::LogConfigBuilder;

use super::cli::CommandLineSettings;

/// Set up the logging infrastructure
pub fn set_up_logging(cli_args: &CommandLineSettings) {
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
}
