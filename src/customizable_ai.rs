use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::{info, error};
use std::result;

/// Customizable AI model
pub struct CustomizableAI {
    core_service: CoreService,
    config: Config,
}

impl CustomizableAI {
    /// Create a new customizable AI instance
    pub fn new(core_service: CoreService, config: Config) -> Self {
        CustomizableAI { core_service, config }
    }

    /// Train the AI model with a given user and institution
    pub fn train(&self, user: User, institution: EducationInstitution) -> result::Result<(), String> {
        info!("Training AI model for user {} and institution {}", user.name, institution.name);
        match self.core_service.train_ai_model(user, institution) {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Error training AI model: {}", err);
                Err("Failed to train AI model".to_string())
            }
        }
    }

    /// Evaluate the AI model with a given user and institution
    pub fn evaluate(&self, user: User, institution: EducationInstitution) -> result::Result<f64, String> {
        info!("Evaluating AI model for user {} and institution {}", user.name, institution.name);
        match self.core_service.evaluate_ai_model(user, institution) {
            Ok(score) => Ok(score),
            Err(err) => {
                error!("Error evaluating AI model: {}", err);
                Err("Failed to evaluate AI model".to_string())
            }
        }
    }
}

struct EducationInstitution {
    id: i32,
    name: String,
    type_: String,
}