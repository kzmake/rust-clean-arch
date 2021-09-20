use anyhow::{Error, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::aggregate::AggregateRoot;
use crate::domain::entity::Entity;
use crate::domain::repository::CreateRepository;
use crate::domain::repository::Repository;

pub struct MemoryRepository<T>
where
    T: AggregateRoot<T> + Entity<T>,
{
    store: Lazy<Arc<Mutex<HashMap<String, T>>>>,
}

impl<T> MemoryRepository<T>
where
    T: AggregateRoot<T> + Entity<T>,
{
    pub fn new() -> Self {
        Self {
            store: Lazy::new(|| {
                let m = HashMap::new();
                Arc::new(Mutex::new(m))
            }),
        }
    }
}

impl<T> Repository for MemoryRepository<T> where T: AggregateRoot<T> + Entity<T> {}

impl<T> CreateRepository<T> for MemoryRepository<T>
where
    T: AggregateRoot<T> + Entity<T>,
{
    fn save(&self, aggregate_root: T) -> Result<(), Error> {
        let mut s = self.store.lock().unwrap();
        s.insert(aggregate_root.id().to_string(), aggregate_root);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::entity::{User, UserBuilder};
    use crate::domain::repository::CreateRepository;
    use crate::domain::vo::{Id, Name};

    #[test]
    fn test_user_save() {
        let sut = MemoryRepository::<User>::default();

        let alice: User = UserBuilder::default()
            .id(Id::<User>::new("01BX5ZZKBKACTAV9WEVGEMMVRZ"))
            .name(Name::<User>::new("alice").unwrap())
            .build()
            .unwrap();

        assert_eq!(sut.save(alice).unwrap(), ());

        let bob: User = UserBuilder::default()
            .id(Id::<User>::new("01BX5ZZKBKACTAV9WEVGEMMVS0"))
            .name(Name::<User>::new("bob").unwrap())
            .build()
            .unwrap();
        assert_eq!(sut.save(bob).unwrap(), ());
    }
}
