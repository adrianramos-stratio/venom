use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

const CONFIG_PATH_ENV: &str = "VULMAN_CONFIG_PATH";
const DEFAULT_CONFIG_FILE_PATH: &str = "config";
const DEFAULT_HOST: &str = "0.0.0.0";

#[derive(Debug, Deserialize)]
pub struct VulmanConfig {
    pub server: Server,
    pub sboms_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl VulmanConfig {
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
