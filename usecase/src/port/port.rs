use anyhow::{Error, Result};

pub trait InputData {}
pub trait OutputData {}

pub trait Port {
    fn handle(&self, input: &dyn InputData) -> Result<&dyn OutputData, Error>;
}
