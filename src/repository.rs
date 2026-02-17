use src::models::User;
use src::database::DB;
use log::info;

/// Data repository for OpenLlama system
pub struct Repository {
    db: DB,
}

impl Repository {
    /// Creates a new instance of the repository
    ///
    /// # Arguments
    ///
    /// * `db` - The database connection
    ///
    /// # Returns
    ///
    /// A new instance of the repository
    pub fn new(db: DB) -> Repository {
        Repository { db }
    }

    /// Retrieves a user from the database
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user
    ///
    /// # Returns
    ///
    /// The user with the specified ID, or an error if not found
    pub fn get_user(&self, id: i32) -> Result<User, String> {
        match self.db.get_user(id) {
            Ok(user) => {
                info!("User with ID {} retrieved successfully", id);
                Ok(user)
            }
            Err(err) => {
                info!("Error retrieving user with ID {}: {}", id, err);
                Err(format!("Error retrieving user: {}", err))
            }
        }
    }

    /// Retrieves all education institutions from the database
    ///
    /// # Returns
    ///
    /// A vector of all education institutions, or an error if not found
    pub fn get_education_institutions(&self) -> Result<Vec<EducationInstitution>, String> {
        match self.db.get_education_institutions() {
            Ok(institutions) => {
                info!("Education institutions retrieved successfully");
                Ok(institutions)
            }
            Err(err) => {
                info!("Error retrieving education institutions: {}", err);
                Err(format!("Error retrieving education institutions: {}", err))
            }
        }
    }
}