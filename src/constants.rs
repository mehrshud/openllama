use std::env;
use std::fmt;

/// Configuration related constants
pub const DEFAULT_DEBUG_MODE: bool = false;
pub const DEFAULT_DB_URL: &str = "sqlite:///openllama.db";

/// Database related constants
pub const DEFAULT_DB_USER: &str = "openllama";
pub const DEFAULT_DB_PASSWORD: &str = "openllama";

/// Service related constants
pub const SERVICE_PORT: u16 = 8080;

/// Error handling function to get environment variable
pub fn get_env_var(var_name: &str) -> Result<String, env::VarError> {
    env::var(var_name)
}

/// Function to get debug mode from environment variable or use default
pub fn get_debug_mode() -> bool {
    match get_env_var("OPENLLAMA_DEBUG_MODE") {
        Ok(val) => val.parse().unwrap_or(DEFAULT_DEBUG_MODE),
        Err(_) => DEFAULT_DEBUG_MODE,
    }
}

/// Function to get database url from environment variable or use default
pub fn get_db_url() -> String {
    match get_env_var("OPENLLAMA_DB_URL") {
        Ok(val) => val,
        Err(_) => DEFAULT_DB_URL.to_string(),
    }
}

/// Function to configure logging
pub fn configure_logging() {
    env_logger::init();
}

/// Custom Error type for OpenLlama application
#[derive(Debug)]
pub enum OpenLlamaError {
    ConfigError(String),
    DatabaseError(String),
}

impl fmt::Display for OpenLlamaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenLlamaError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            OpenLlamaError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl std::error::Error for OpenLlamaError {}

pub fn get_default_config() -> Config {
    Config {
        debug: DEFAULT_DEBUG_MODE,
        db_url: DEFAULT_DB_URL.to_string(),
    }
}