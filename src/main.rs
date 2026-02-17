use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::info;
use std::env;

/// Entry point of the OpenLlama system.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load configuration from environment variables
    let debug = env::var("OPENLLAMA_DEBUG").unwrap_or("false".to_string()) == "true";
    let db_url = env::var("OPENLLAMA_DB_URL").expect("OPENLLAMA_DB_URL must be set");

    // Create configuration struct
    let config = Config { debug, db_url };

    // Create a new CoreService instance
    let core_service = CoreService::new(config)?;

    // Create a sample user
    let user = User { id: 1, name: "John Doe".to_string(), email: "john.doe@example.com".to_string() };

    // Log user information
    info!("User: {:?}", user);

    // Call the process_user method on the CoreService instance
    core_service.process_user(user)?;

    Ok(())
}