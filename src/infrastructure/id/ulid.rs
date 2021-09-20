use crate::domain::repository::IdRepository;
use crate::domain::repository::Repository;
use anyhow::{Error, Result};
use ulid::Ulid;

pub struct UlidRepository {}

impl UlidRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl Repository for UlidRepository {}

impl IdRepository for UlidRepository {
    fn generate(&self) -> Result<String, Error> {
        Ok(Ulid::new().to_string())
    }
}
