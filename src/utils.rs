use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::{info, error};
use std::result::Result;

/// Utility functions for OpenLlama.
pub mod utils {
    /// Logs and returns a formatted error message with a given context and error.
    ///
    /// # Arguments
    ///
    /// * `context`: Context in which the error occurred.
    /// * `error`: Error that occurred.
    ///
    /// # Returns
    ///
    /// * `String`: Formatted error message.
    pub fn log_and_return_error(context: &str, error: &dyn std::error::Error) -> String {
        let error_message = format!("{}: {}", context, error);
        error!("{}", error_message);
        error_message
    }

    /// Checks if a user exists in the system.
    ///
    /// # Arguments
    ///
    /// * `core_service`: Core service instance.
    /// * `user_id`: ID of the user to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, String>`: Whether the user exists, or an error message.
    pub fn check_user_exists(core_service: &CoreService, user_id: i32) -> Result<bool, String> {
        let result = core_service.get_user(user_id);
        match result {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(error) => Err(log_and_return_error("Failed to check user existence", &error)),
        }
    }

    /// Generates a unique user ID.
    ///
    /// # Returns
    ///
    /// * `i32`: Unique user ID.
    pub fn generate_unique_user_id() -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen::<i32>()
    }
}