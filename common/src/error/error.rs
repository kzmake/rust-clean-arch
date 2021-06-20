use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid {field_name:?}: {found:?}")]
    Invalid { field_name: String, found: String },
}

impl DomainError {
    pub fn invalid_user_name(found: &str) -> DomainError {
        DomainError::Invalid {
            field_name: "user_name".to_string(),
            found: found.to_string(),
        }
    }
}
