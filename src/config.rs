use crate::models::{User, EducationInstitution};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Configuration management for OpenLlama application.
pub struct Config {
    /// Debug mode flag.
    pub debug: bool,
    /// Database URL.
    pub db_url: String,
}

impl Config {
    /// Load configuration from environment variables and configuration file.
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let debug = env::var("OPENLLAMA_DEBUG").map_or(false, |v| v.to_lowercase() == "true");
        let db_url = env::var("OPENLLAMA_DB_URL").expect("OPENLLAMA_DB_URL environment variable is not set");

        if debug {
            println!("Debug mode enabled");
        }

        Ok(Config { debug, db_url })
    }

    /// Save configuration to a file.
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let config_dir = Path::new(path).parent().unwrap();
        fs::create_dir_all(config_dir)?;

        let config_file = format!("{}{}", path, "/config.toml");
        let toml_config = format!("debug = {}\ndb_url = {}", self.debug, self.db_url);

        fs::write(config_file, toml_config)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            debug: false,
            db_url: String::from(""),
        }
    }
}