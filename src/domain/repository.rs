use crate::domain::aggregate::AggregateRoot;
use crate::domain::entity::Entity;
use anyhow::{Error, Result};

pub trait Repository {}

pub trait IdRepository: Repository {
    fn generate(&self) -> Result<String, Error>;
}

pub trait HaveIdRepository {
    type IdRepository: IdRepository;

    fn provide_id_repository(&self) -> &Self::IdRepository;
}

pub trait CreateRepository<T: AggregateRoot<T> + Entity<T>>: Repository {
    fn save(&self, aggregate_root: T) -> Result<(), Error>;
}

pub trait HaveCreateRepository<T: AggregateRoot<T> + Entity<T>> {
    type CreateRepository: CreateRepository<T>;
    fn provide_repository(&self) -> Self::CreateRepository;
}
