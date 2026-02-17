use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use src::database::DB;
use log::{info, error};

/// Research-oriented service layer
pub struct ResearchService {
    db: DB,
    config: Config,
}

impl ResearchService {
    /// Create a new ResearchService instance
    pub fn new(db: DB, config: Config) -> Self {
        ResearchService { db, config }
    }

    /// Perform research on a specific education institution
    pub fn perform_research(&self, institution: EducationInstitution) -> Result<(), String> {
        info!("Performing research on institution: {}", institution.name);
        let users = self.core_service().get_users(institution.id);
        match users {
            Ok(users) => {
                for user in users {
                    info!("Processing user: {}", user.name);
                    // Perform research on each user
                }
                Ok(())
            }
            Err(e) => {
                error!("Error performing research: {}", e);
                Err("Error performing research".to_string())
            }
        }
    }

    /// Get the core service instance
    fn core_service(&self) -> CoreService {
        CoreService::new(self.db.clone(), self.config.clone())
    }
}