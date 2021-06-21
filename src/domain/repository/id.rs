use anyhow::{Error, Result};

pub trait IdRepository {
    fn generate(&self) -> Result<String, Error>;
}

pub trait HaveIdRepository {
    type IdRepository: IdRepository;

    fn provide_id_repository(&self) -> &Self::IdRepository;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_generate() {
//         use anyhow::{Error, Result};

//         const ID: &str = "01F8MECHZX3TBDSZ7XRADM79XE";

//         pub trait MockIdRepository {}

//         impl<T: MockIdRepository> IdRepository for T {
//             fn generate(&self) -> Result<String, Error> {
//                 Ok(ID.to_string())
//             }
//         }

//         #[derive(Default)]
//         struct Container {}

//         impl MockIdRepository for Container {}

//         impl HaveIdRepository for Container {
//             type IdRepository = Self;
//             fn provide_id_repository(&self) -> &Self::IdRepository {
//                 &self
//             }
//         }

//         let ctn = Container::default();
//         let sut = ctn.provide_id_repository();

//         // ok
//         assert_eq!(sut.generate().unwrap(), ID.to_string());
//     }
// }
