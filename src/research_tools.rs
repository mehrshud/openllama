use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::{debug, error};

/// Provides research-oriented tools for the OpenLlama system.
pub struct ResearchTools {
    core_service: CoreService,
    config: Config,
}

impl ResearchTools {
    /// Creates a new instance of the ResearchTools struct.
    /// 
    /// # Arguments
    /// 
    /// * `core_service` - An instance of the CoreService.
    /// * `config` - An instance of the Config.
    /// 
    /// # Returns
    /// 
    /// A new instance of the ResearchTools struct.
    pub fn new(core_service: CoreService, config: Config) -> Self {
        ResearchTools { core_service, config }
    }

    /// Retrieves a list of all users from the database.
    /// 
    /// # Returns
    /// 
    /// A Vec of User instances.
    pub fn get_all_users(&self) -> Vec<User> {
        match self.core_service.get_all_users() {
            Ok(users) => {
                debug!("Successfully retrieved all users.");
                users
            }
            Err(err) => {
                error!("Failed to retrieve all users: {}", err);
                Vec::new()
            }
        }
    }

    /// Retrieves a user by their ID from the database.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The ID of the user to retrieve.
    /// 
    /// # Returns
    /// 
    /// An Option containing a User instance, or None if the user was not found.
    pub fn get_user_by_id(&self, id: i32) -> Option<User> {
        match self.core_service.get_user_by_id(id) {
            Ok(user) => {
                debug!("Successfully retrieved user by ID.");
                Some(user)
            }
            Err(err) => {
                error!("Failed to retrieve user by ID: {}", err);
                None
            }
        }
    }
}