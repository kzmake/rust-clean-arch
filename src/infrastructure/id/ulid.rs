use anyhow::{Error, Result};
use ulid::Ulid;

use crate::domain::repository::IdRepository;

pub trait UlidRepository {}

impl<T: UlidRepository> IdRepository for T {
    fn generate(&self) -> Result<String, Error> {
        Ok(Ulid::new().to_string())
    }
}
