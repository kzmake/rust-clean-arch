use anyhow::{Error, Result};

use crate::domain::aggregate::AggregateRoot;

pub trait Repository<T: AggregateRoot> {
    fn save(&self, resource: T) -> Result<(), Error>;
}

pub trait HaveRepository<T: AggregateRoot> {
    type Repository: Repository<T>;

    fn provide_repository(&self) -> &Self::Repository;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::domain::aggregate::*;
//     use crate::domain::vo::*;

//     #[test]
//     fn test_generate() {
//         use anyhow::{Error, Result};

//         pub trait MockUserRepository {}

//         impl<T: MockUserRepository> UserRepository for T {
//             fn save(&self, _: User) -> Result<(), Error> {
//                 Ok(())
//             }
//         }

//         #[derive(Default)]
//         struct Registry {}

//         impl MockUserRepository for Registry {}

//         impl HaveUserRepository for Registry {
//             type UserRepository = Self;
//             fn provide_user_repository(&self) -> &Self::UserRepository {
//                 &self
//             }
//         }

//         let reg = Registry::default();
//         let sut = reg.provide_user_repository();

//         let user: User = UserBuilder::default()
//             .id(Id::<User>::new("01F8MECHZX3TBDSZ7XRADM79XE"))
//             .name(Name::<User>::new("hoge").unwrap())
//             .build()
//             .unwrap();

//         // ok
//         assert_eq!(sut.save(user).unwrap(), ());
//     }
// }
