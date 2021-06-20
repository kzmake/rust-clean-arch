use anyhow::{bail, Error, Result};
use common::error::DomainError;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn new(s: &str) -> Result<Self, Error> {
        if s.chars().count() < 4 {
            bail!(DomainError::invalid_user_name(s))
        }

        Ok(UserName(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::error::DomainError;

    #[test]
    fn test_user_name() {
        // ok
        assert_eq!(UserName::new("hoge").unwrap(), UserName("hoge".to_string()));
        assert_eq!(
            UserName::new("fugafuga").unwrap(),
            UserName("fugafuga".to_string())
        );
        assert_eq!(
            UserName::new("piyopiyopiyo").unwrap(),
            UserName("piyopiyopiyo".to_string())
        );

        // ko
        assert_eq!(
            UserName::new("ko").unwrap_err().to_string(),
            DomainError::invalid_user_name("ko").to_string(),
        );
    }
}
