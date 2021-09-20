use crate::domain::entity::{User, UserBuilder};
use crate::domain::repository::CreateRepository;
use crate::domain::repository::IdRepository;
use crate::domain::repository::Repository;
use crate::domain::vo::{Id, Name};
use crate::usecase::port::{CreateUserInputData, CreateUserOutputData, CreateUserPort};
use anyhow::{Error, Result};

pub struct CreateUserInteractor<T, U>
where
    T: Repository + IdRepository,
    U: Repository + CreateRepository<User>,
{
    id_repository: T,
    user_repository: U,
}

impl<T, U> CreateUserInteractor<T, U>
where
    T: Repository + IdRepository,
    U: Repository + CreateRepository<User>,
{
    pub fn new(id_repository: T, user_repository: U) -> Self {
        Self {
            id_repository,
            user_repository,
        }
    }
}

impl<T, U> CreateUserPort for CreateUserInteractor<T, U>
where
    T: Repository + IdRepository,
    U: Repository + CreateRepository<User>,
{
    fn handle(&self, input: CreateUserInputData) -> Result<CreateUserOutputData, Error> {
        let new_id: String = self.id_repository.generate()?;

        let user: User = UserBuilder::default()
            .id(Id::<User>::new(new_id.as_str()))
            .name(Name::<User>::new(input.user_name.as_str())?)
            .build()?;

        self.user_repository.save(user.clone())?;

        Ok(CreateUserOutputData::new(
            user.id().to_string(),
            user.name().to_string(),
        ))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_handle() {
//         use anyhow::{Error, Result};

//         use crate::domain::repository::{HaveIdRepository, IdRepository};
//         use crate::usecase::port::{CreateUserPort, HaveCreateUserPort};

//         const ID: &str = "01F8MECHZX3TBDSZ7XRADM79XE";

//         #[derive(Default)]
//         struct Registry {}

//         // mock
//         impl IdRepository for Registry {
//             fn generate(&self) -> Result<String, Error> {
//                 Ok(ID.to_string())
//             }
//         }

//         impl CreateUserInteractor for Registry {}

//         impl HaveIdRepository for Registry {
//             type IdRepository = Self;
//             fn provide_id_repository(&self) -> &Self::IdRepository {
//                 &self
//             }
//         }

//         impl HaveCreateUserPort for Registry {
//             type CreateUserPort = Self;
//             fn provide_create_user_port(&self) -> &Self::CreateUserPort {
//                 &self
//             }
//         }

//         let reg: Registry = Registry::default();
//         let sut = CreateUserInteractor::new(
// UlidRepository,
// user_repository,
// )

//         assert_eq!(
//             sut.handle(CreateUserInputData::new("hoge".to_string()))
//                 .unwrap(),
//             CreateUserOutputData::new(ID.to_string(), "hoge".to_string())
//         );
//     }
// }
