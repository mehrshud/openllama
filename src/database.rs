use src::models::{User, EducationInstitution};
use src::config::Config;
use sqlx::PgPool;
use log::info;

pub struct DB {
    pool: PgPool,
}

impl DB {
    pub async fn new(config: &Config) -> Result<DB, sqlx::Error> {
        let pool = PgPool::connect(&config.db_url).await?;
        Ok(DB { pool })
    }

    pub async fn get_user(&self, id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_institution(&self, id: i32) -> Result<EducationInstitution, sqlx::Error> {
        let institution = sqlx::query_as!(
            EducationInstitution,
            "SELECT id, name, type FROM institutions WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(institution)
    }

    pub async fn create_user(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
            user.id,
            user.name,
            user.email
        )
        .execute(&self.pool)
        .await?;
        info!("User created: {}", user.name);
        Ok(())
    }

    pub async fn create_institution(&self, institution: EducationInstitution) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO institutions (id, name, type) VALUES ($1, $2, $3)",
            institution.id,
            institution.name,
            institution.type
        )
        .execute(&self.pool)
        .await?;
        info!("Institution created: {}", institution.name);
        Ok(())
    }
}