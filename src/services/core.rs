use src::models::User;
use src::database::DB;
use log::{info, error};

/// Core Service Layer
pub struct CoreService {
    db: DB,
}

impl CoreService {
    /// Creates a new instance of CoreService
    pub fn new(db: DB) -> Self {
        CoreService { db }
    }

    /// Retrieves a user by their id
    pub fn get_user(&self, id: i32) -> Result<User, String> {
        let result = self.db.get_user(id);
        match result {
            Ok(user) => {
                info!("User retrieved successfully");
                Ok(user)
            }
            Err(err) => {
                error!("Error retrieving user: {}", err);
                Err(err.to_string())
            }
        }
    }

    /// Creates a new user
    pub fn create_user(&self, user: User) -> Result<(), String> {
        let result = self.db.create_user(user);
        match result {
            Ok(_) => {
                info!("User created successfully");
                Ok(())
            }
            Err(err) => {
                error!("Error creating user: {}", err);
                Err(err.to_string())
            }
        }
    }
}