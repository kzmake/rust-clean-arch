use anyhow::{Error, Result};
use derive_new::new;

use crate::usecase::port::{InputData, OutputData};

pub trait CreateUserPort {
    fn handle(&self, input: CreateUserInputData) -> Result<CreateUserOutputData, Error>;
}

pub trait HaveCreateUserPort {
    type CreateUserPort: CreateUserPort;

    fn provide_create_user_port(&self) -> &Self::CreateUserPort;
}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct CreateUserInputData {
    pub user_name: String,
}

impl InputData for CreateUserInputData {}

#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct CreateUserOutputData {
    pub user_id: String,
    pub user_name: String,
}

impl OutputData for CreateUserOutputData {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        use anyhow::{Error, Result};

        #[derive(Default)]
        struct Registry {}

        impl CreateUserPort for Registry {
            fn handle(&self, _input: CreateUserInputData) -> Result<CreateUserOutputData, Error> {
                Ok(CreateUserOutputData {
                    user_id: "01F8MECHZX3TBDSZ7XRADM79XE".to_string(),
                    user_name: "hoge".to_string(),
                })
            }
        }

        impl HaveCreateUserPort for Registry {
            type CreateUserPort = Self;
            fn provide_create_user_port(&self) -> &Self::CreateUserPort {
                &self
            }
        }

        let reg = Registry::default();
        let sut = reg.provide_create_user_port();

        // ok
        assert_eq!(
            sut.handle(CreateUserInputData::new("hoge".to_string()))
                .unwrap(),
            CreateUserOutputData::new("01F8MECHZX3TBDSZ7XRADM79XE".to_string(), "hoge".to_string())
        );
    }

    #[test]
    fn test_input_data() {
        assert_eq!(
            CreateUserInputData::new("hoge".to_string()),
            CreateUserInputData {
                user_name: "hoge".to_string(),
            }
        );
    }

    #[test]
    fn test_output_data() {
        assert_eq!(
            CreateUserOutputData::new("01F8MECHZX3TBDSZ7XRADM79XE".to_string(), "hoge".to_string()),
            CreateUserOutputData {
                user_id: "01F8MECHZX3TBDSZ7XRADM79XE".to_string(),
                user_name: "hoge".to_string(),
            }
        );
    }
}
