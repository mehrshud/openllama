use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::{info, error};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ApiServiceError {
    message: String,
}

impl fmt::Display for ApiServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ApiServiceError {}

pub struct ApiGateway {
    core_service: CoreService,
    config: Config,
}

impl ApiGateway {
    pub fn new(config: Config) -> Self {
        ApiGateway {
            core_service: CoreService::new(),
            config,
        }
    }

    pub fn handle_request(&self, user: User) -> Result<String, ApiServiceError> {
        info!("Received request from user: {}", user.name);
        match self.core_service.process_request(user) {
            Ok(response) => {
                info!("Request processed successfully");
                Ok(response)
            }
            Err(err) => {
                error!("Error processing request: {}", err);
                Err(ApiServiceError {
                    message: "Error processing request".to_string(),
                })
            }
        }
    }
}