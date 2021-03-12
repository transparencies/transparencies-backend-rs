//! Everything regarding the commandline interface

use structopt::StructOpt;

pub trait CommandLineSetting {
    fn debug(&self) -> bool;
    fn log_file_path(&self) -> String;
    fn log_level(&self) -> String;
}
impl CommandLineSetting for CommandLineSettings {
    fn debug(&self) -> bool {
        self.debug
    }

    fn log_file_path(&self) -> String {
        self.log_file_path.to_owned()
    }

    fn log_level(&self) -> String {
        self.log_level.to_owned()
    }
}
impl CommandLineSetting for ExportCommandLineSettings {
    fn debug(&self) -> bool {
        self.debug
    }

    fn log_file_path(&self) -> String {
        self.log_file_path.to_owned()
    }

    fn log_level(&self) -> String {
        self.log_level.to_owned()
    }
}

/// `StructOpt`'s struct for parsing commandline input
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
    #[structopt(
        long = "log-file",
        default_value = "./logs/transparencies.log"
    )]
    pub log_file_path: String,

    /// Log level
    #[structopt(long = "log-level", default_value = "debug")]
    pub log_level: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}
/// `StructOpt`'s struct for parsing commandline input
#[derive(StructOpt, Debug, serde::Deserialize)]
#[structopt(name = "transparencies export tool", about = "Export test data")]
pub struct ExportCommandLineSettings {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Log file path
    #[structopt(
        long = "log-file",
        default_value = "./logs/export-test-data.log"
    )]
    pub log_file_path: String,

    /// Log level
    #[structopt(long = "log-level", default_value = "debug")]
    pub log_level: String,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Test case export path
    #[structopt(
        short = "tcf",
        long = "test-case-folder",
        default_value = "./tests/matchinfo-integration/test_case_template"
    )]
    pub test_case_export_path: String,
}
