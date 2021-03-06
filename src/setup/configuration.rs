//! Collects all the configurational settings done with `config` crate

use serde_aux::field_attributes::deserialize_number_from_string;
use std::convert::{
    TryFrom,
    TryInto,
};
/// Contains server settings (e.g. port, hostname)
#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    /// Port the server listens to
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    /// IP address the server runs on
    pub host: String,
}

/// Container for all our settings
#[derive(serde::Deserialize)]
pub struct Settings {
    /// Settings regarding the application logic
    pub application: ApplicationSettings,
}

/// Parses the settings from our configuration files and returns a `Settings`
/// struct
///
/// # Errors
// TODO
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Read the "default" configuration file
    settings.merge(
        config::File::from(configuration_directory.join("base")).required(true),
    )?;

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Layer on the environment-specific values.
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str()))
            .required(true),
    )?;

    // Add in settings from environment variables (with a prefix of APP and '__'
    // as separator) E.g. `APP_APPLICATION__PORT=5001 would set
    // `Settings.application.port`
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

/// The possible runtime environment for our application
pub enum Environment {
    /// Local for testing purposes
    Local,
    /// Production for running 24/7
    Production,
}

impl Environment {
    /// Get &str Value of Enum
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
