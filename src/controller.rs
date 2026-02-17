use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::error;

/// Controller for handling requests related to users and education institutions.
pub struct Controller {
    config: Config,
    core_service: CoreService,
}

impl Controller {
    /// Creates a new instance of the Controller.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the controller.
    /// * `core_service` - The core service instance.
    pub fn new(config: Config, core_service: CoreService) -> Self {
        Self { config, core_service }
    }

    /// Retrieves a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<User, String>` - The retrieved user or an error message.
    pub fn get_user(&self, id: i32) -> Result<User, String> {
        match self.core_service.get_user(id) {
            Ok(user) => Ok(user),
            Err(err) => {
                error!("Error retrieving user: {}", err);
                Err("Error retrieving user".to_string())
            }
        }
    }

    /// Creates a new user.
    ///
    /// # Arguments
    ///
    /// * `user` - The user to create.
    ///
    /// # Returns
    ///
    /// * `Result<User, String>` - The created user or an error message.
    pub fn create_user(&self, user: User) -> Result<User, String> {
        match self.core_service.create_user(user) {
            Ok(created_user) => Ok(created_user),
            Err(err) => {
                error!("Error creating user: {}", err);
                Err("Error creating user".to_string())
            }
        }
    }
}