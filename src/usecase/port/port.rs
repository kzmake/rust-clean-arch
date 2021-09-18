use anyhow::{Error, Result};

pub trait Usecase {}

pub trait InputData<T: Usecase> {}
pub trait OutputData<T: Usecase> {}

pub trait Port<T: Usecase> {
    fn handle(&self, input: &dyn InputData<T>) -> Result<&dyn OutputData<T>, Error>;
}
