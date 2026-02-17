use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::{info, error};

/// LlamaModel provides the functionality to make predictions and perform other AI-related tasks.
pub struct LlamaModel {
    config: Config,
}

impl LlamaModel {
    /// Creates a new instance of LlamaModel with the given configuration.
    pub fn new(config: Config) -> Self {
        LlamaModel { config }
    }

    /// Makes a prediction based on the given user and institution.
    pub fn make_prediction(&self, user: User, institution: EducationInstitution) -> Result<String, String> {
        info!("Making prediction for user {} and institution {}", user.name, institution.name);
        let core_service = CoreService::new(self.config.clone());
        match core_service.get_data(user, institution) {
            Ok(data) => {
                info!("Data retrieved successfully");
                Ok(self.process_data(data))
            }
            Err(err) => {
                error!("Failed to retrieve data: {}", err);
                Err("Failed to retrieve data".to_string())
            }
        }
    }

    /// Processes the retrieved data to make a prediction.
    fn process_data(&self, data: String) -> String {
        info!("Processing data to make a prediction");
        // Implement the data processing logic here
        // For demonstration purposes, a simple string is returned
        "Prediction made successfully".to_string()
    }
}

/// EducationInstitution represents an educational institution.
#[derive(Debug)]
pub struct EducationInstitution {
    pub id: i32,
    pub name: String,
    pub type_: String,
}