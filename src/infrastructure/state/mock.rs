use anyhow::{Error, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::aggregate::User;
use crate::domain::repository::UserRepository;

pub trait MockUserRepository {}

impl<T: MockUserRepository> UserRepository for T {
    fn save(&self, user: User) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::aggregate::{User, UserBuilder};
    use crate::domain::repository::{HaveUserRepository, UserRepository};
    use crate::domain::vo::{Id, Name};

    #[test]
    fn test_save() {
        #[derive(Default)]
        struct Registry {}

        impl MockUserRepository for Registry {}

        impl HaveUserRepository for Registry {
            type UserRepository = Self;
            fn provide_user_repository(&self) -> &Self::UserRepository {
                &self
            }
        }

        let reg: Registry = Registry::default();
        let sut = reg.provide_user_repository();

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
