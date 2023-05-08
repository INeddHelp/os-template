use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub hostname: String,
    pub timezone: String,
    pub language: String,
    pub log_level: LogLevel,
    // ... other configuration options ...
    // TODO: Add configuration options for:
    // - Keyboard layout
    // - Timezone
    // - Language
    // - Log level
    // - ...
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hostname: "localhost".to_string(),
            timezone: "UTC".to_string(),
            language: "en_US".to_string(),
            log_level: LogLevel::Info,
            // ... set default values for other configuration options ...
            // TODO: Set default values for:
            // - Keyboard layout
            // - Timezone
            // - Language
            // - Log level
            // - ...
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // Load configuration from file or other source
        // For example, using serde to deserialize from a JSON file
        let config_file_path = "/etc/my_os/config.json"; // TODO: Use a more appropriate path
        // TODO: create a folder tree
        let config_file = std::fs::File::open(config_file_path)?;
        let config: Config = serde_json::from_reader(config_file)?;

        Ok(config)
    }
}

// TODO: Create a custom error type for configuration errors

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::Io(error)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        ConfigError::Serde(error)
    }
}
