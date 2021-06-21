use anyhow::{bail, Error, Result};
use getset::Getters;
use std::fmt;
use std::marker::PhantomData;

use common::error::DomainError;

#[derive(Default, Clone, Debug, PartialEq, Eq, Getters)]
pub struct Name<T> {
    #[getset(get = "pub")]
    value: String,

    _phantom: PhantomData<T>,
}

impl<T> Name<T> {
    pub fn new(s: &str) -> Result<Self, Error> {
        if s.chars().count() <= 2 {
            bail!(DomainError::invalid_user_name(s))
        }

        Ok(Name {
            value: s.to_string(),
            _phantom: PhantomData,
        })
    }
}

impl<T> fmt::Display for Name<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use common::error::DomainError;

    #[derive(Debug, PartialEq)]
    struct User {}

    #[test]
    fn test_name() {
        // ok
        assert_eq!(
            Name::<User>::new("hoge").unwrap(),
            Name {
                value: "hoge".to_string(),
                _phantom: PhantomData,
            }
        );
        assert_eq!(
            Name::<User>::new("fugafuga").unwrap(),
            Name {
                value: "fugafuga".to_string(),
                _phantom: PhantomData,
            }
        );
        assert_eq!(
            Name::<User>::new("piyopiyopiyo").unwrap(),
            Name {
                value: "piyopiyopiyo".to_string(),
                _phantom: PhantomData,
            }
        );

        // ko
        assert_eq!(
            Name::<User>::new("ko").unwrap_err().to_string(),
            DomainError::invalid_user_name("ko").to_string(),
        );
    }

    #[test]
    fn test_to_string() {
        // ok
        assert_eq!(
            Name::<User>::new("hoge").unwrap().to_string(),
            "hoge".to_string()
        );
    }
}
