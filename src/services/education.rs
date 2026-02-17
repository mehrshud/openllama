use src::models::User;
use src::config::Config;
use src::services::core::CoreService;
use src::database::DB;
use log::info;

pub struct EducationService {
    core_service: CoreService,
    db: DB,
}

impl EducationService {
    pub fn new(core_service: CoreService, db: DB) -> Self {
        EducationService { core_service, db }
    }

    /// Retrieves education institutions for a given user
    pub async fn get_institutions(&self, user: &User) -> Result<Vec<EducationInstitution>, String> {
        info!("Getting education institutions for user {}", user.id);
        let institutions = self.db.get_institutions(user.id).await;
        match institutions {
            Ok(data) => Ok(data),
            Err(err) => {
                error!("Error retrieving institutions: {}", err);
                Err("Failed to retrieve institutions".to_string())
            }
        }
    }

    /// Creates a new education institution
    pub async fn create_institution(&self, institution: &EducationInstitution) -> Result<i32, String> {
        info!("Creating new education institution");
        let institution_id = self.db.create_institution(institution).await;
        match institution_id {
            Ok(id) => Ok(id),
            Err(err) => {
                error!("Error creating institution: {}", err);
                Err("Failed to create institution".to_string())
            }
        }
    }
}

pub struct EducationInstitution {
    pub id: i32,
    pub name: String,
    pub type_: String,
}