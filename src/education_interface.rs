use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::{info, error};

/// Education-focused interface for interacting with the OpenLlama system.
pub struct EducationInterface {
    core_service: CoreService,
    config: Config,
}

impl EducationInterface {
    /// Initialize the education interface.
    pub fn new(core_service: CoreService, config: Config) -> Self {
        EducationInterface { core_service, config }
    }

    /// Get a user by their ID.
    pub fn get_user(&self, user_id: i32) -> Result<User, String> {
        match self.core_service.get_user(user_id) {
            Ok(user) => {
                info!("Retrieved user with ID: {}", user_id);
                Ok(user)
            }
            Err(e) => {
                error!("Failed to retrieve user: {}", e);
                Err(format!("Failed to retrieve user: {}", e))
            }
        }
    }

    /// Get all users associated with an education institution.
    pub fn get_users_by_institution(&self, institution: EducationInstitution) -> Result<Vec<User>, String> {
        match self.core_service.get_users_by_institution(institution) {
            Ok(users) => {
                info!("Retrieved users for institution with ID: {}", institution.id);
                Ok(users)
            }
            Err(e) => {
                error!("Failed to retrieve users: {}", e);
                Err(format!("Failed to retrieve users: {}", e))
            }
        }
    }
}