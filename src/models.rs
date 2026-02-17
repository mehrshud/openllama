use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
/// Represents a User in the system
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents a Config in the system
pub struct Config {
    pub debug: bool,
    pub db_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents an Education Institution in the system
pub struct EducationInstitution {
    pub id: i32,
    pub name: String,
    pub institution_type: String,
}

impl User {
    /// Creates a new User instance
    pub fn new(id: i32, name: String, email: String) -> Self {
        User { id, name, email }
    }
}

impl Config {
    /// Creates a new Config instance
    pub fn new(debug: bool, db_url: String) -> Self {
        Config { debug, db_url }
    }
}

impl EducationInstitution {
    /// Creates a new Education Institution instance
    pub fn new(id: i32, name: String, institution_type: String) -> Self {
        EducationInstitution { id, name, institution_type }
    }
}

/// Function to validate the User model
pub fn validate_user(user: &User) -> Result<(), Box<dyn Error>> {
    if user.id <= 0 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "User ID must be positive")));
    }
    if user.name.is_empty() || user.email.is_empty() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Name and Email cannot be empty")));
    }
    Ok(())
}

/// Function to validate the Config model
pub fn validate_config(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.db_url.is_empty() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Database URL cannot be empty")));
    }
    Ok(())
}

/// Function to validate the Education Institution model
pub fn validate_institution(institution: &EducationInstitution) -> Result<(), Box<dyn Error>> {
    if institution.id <= 0 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Institution ID must be positive")));
    }
    if institution.name.is_empty() || institution.institution_type.is_empty() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Name and Institution Type cannot be empty")));
    }
    Ok(())
}