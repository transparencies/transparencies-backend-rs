//! Everything regarding the commandline interface

use std::path::PathBuf;
use structopt::StructOpt;

/// StructOpt's struct for parsing commandline input
#[derive(StructOpt, Debug, serde::Deserialize)]
#[structopt(
    name = "transparencies-backend-rs",
    about = "Backend for dynamic stream overlays"
)]
pub struct CommandLineSettings {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Log file path
    #[structopt(long = "log-file", default_value = "./logs/aoe-data-util.log")]
    pub log_file_path: String,

    /// Log file path
    #[structopt(long = "log-level", default_value = "debug")]
    pub log_level: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}
