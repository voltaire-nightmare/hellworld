extern crate serde_json;

use serde::Deserialize;
use std::fs;

pub enum ConfigError {
    ReadError,
    DeserializeError,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub output_string: String,
}

/// Loads a configuration file from its path
pub fn load_config_from_path(filepath: String) -> Result<Config, ConfigError> {
    // Load the file into a string and then deserialize into a config struct
    let fstring: String = match fs::read_to_string(filepath.clone()) {
        Ok(v) => v,
        Err(_) => {
            error!("\"{filepath}\" could not be read");
            return Err(ConfigError::ReadError);
        }
    };

    return match serde_json::from_str::<Config>(fstring.as_str()) {
        Ok(v) => Ok(v),
        Err(_) => {
            error!("\"{filepath}\" could not be deserialized");
            Err(ConfigError::DeserializeError)
        }
    };
}
