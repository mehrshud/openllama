use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use log::info;
use log::error;

/// Record of choosing Rust, TensorFlow, and PyTorch
pub struct ADR_001 {
    pub rationale: String,
}

impl ADR_001 {
    /// Creates a new ADR_001 instance
    pub fn new(rationale: String) -> Self {
        Self { rationale }
    }

    /// Logs the rationale for choosing Rust, TensorFlow, and PyTorch
    pub fn log_rationale(&self) -> Result<(), String> {
        info!("Choosing Rust, TensorFlow, and PyTorch for OpenLlama project");
        info!("Rationale: {}", self.rationale);
        Ok(())
    }

    /// Validates the rationale
    pub fn validate_rationale(&self) -> Result<(), String> {
        if self.rationale.is_empty() {
            let err_msg = "Rationale cannot be empty";
            error!("{}", err_msg);
            return Err(err_msg.to_string());
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let config = Config {
        debug: true,
        db_url: "localhost:5432".to_string(),
    };

    let user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "johndoe@example.com".to_string(),
    };

    let education_institution = src::models::EducationInstitution {
        id: 1,
        name: "Example University".to_string(),
        type_: "University".to_string(),
    };

    let core_service = CoreService::new(config, user, education_institution);

    let adr_001 = ADR_001::new(
        "Rust provides a strong focus on safety and performance, while TensorFlow and PyTorch provide industry-leading AI capabilities.".to_string(),
    );

    if let Err(e) = adr_001.validate_rationale() {
        return Err(e);
    }

    if let Err(e) = adr_001.log_rationale() {
        return Err(e);
    }

    Ok(())
}