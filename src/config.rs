use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

const CONFIG_PATH_ENV: &str = "VULMAN_CONFIG_PATH";
const DEFAULT_CONFIG_FILE_PATH: &str = "config";
const DEFAULT_HOST: &str = "0.0.0.0";

#[derive(Debug, Deserialize)]
pub struct VenomConfig {
    pub server: Server,
    pub sboms_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl VenomConfig {
    /// Load the application configuration from a file and environment variables.
    ///
    /// This method:
    /// - Reads the config file path from the environment variable `VULMAN_CONFIG_PATH`
    ///   (or falls back to the default `"config"`),
    /// - Parses a configuration file using [`config::File`],
    /// - Allows overrides from environment variables prefixed with `VULMAN__`,
    /// - Applies default values like `server.host = "0.0.0.0"`,
    /// - Deserializes the full configuration into [`VulmanConfig`].
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError`] if:
    /// - The config file cannot be read (e.g. does not exist or has invalid syntax),
    /// - The environment variable override is malformed,
    /// - Any required field is missing or fails deserialization.
    ///
    /// See [`config::ConfigError`] for full details.
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = std::env::var(CONFIG_PATH_ENV)
            .unwrap_or_else(|_| String::from(DEFAULT_CONFIG_FILE_PATH));

        let config = Config::builder()
            .set_default("server.host", DEFAULT_HOST.to_string())?
            .add_source(File::with_name(config_path.as_str()))
            // Allow environment variables to set/override config parsing '__' as '.'
            // Keep '_' is needed due to attribute names
            .add_source(Environment::with_prefix("VULMAN").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
