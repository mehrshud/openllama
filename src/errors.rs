use src::models::User;
use src::config::Config;
use src::database::DB;
use log::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OpenLlamaError {
    DatabaseError(String),
    ConfigError(String),
    UserServiceError(String),
}

impl fmt::Display for OpenLlamaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenLlamaError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            OpenLlamaError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            OpenLlamaError::UserServiceError(msg) => write!(f, "User service error: {}", msg),
        }
    }
}

impl Error for OpenLlamaError {}

impl From<DB> for OpenLlamaError {
    fn from(err: DB) -> Self {
        OpenLlamaError::DatabaseError(err.to_string())
    }
}

pub fn handle_error(err: OpenLlamaError) {
    error!("Error occurred: {}", err);
    match err {
        OpenLlamaError::DatabaseError(msg) => error!("Database error: {}", msg),
        OpenLlamaError::ConfigError(msg) => error!("Config error: {}", msg),
        OpenLlamaError::UserServiceError(msg) => error!("User service error: {}", msg),
    }
}