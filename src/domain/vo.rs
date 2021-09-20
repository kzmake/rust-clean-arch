use crate::domain::error::DomainError;
use anyhow::{bail, Error, Result};
use getset::Getters;
use std::fmt;
use std::marker::PhantomData;

#[derive(Default, Clone, Debug, PartialEq, Eq, Getters)]
pub struct Id<T> {
    #[getset(get = "pub")]
    value: String,

    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: &str) -> Self {
        Self {
            value: id.to_string(),
            _phantom: PhantomData,
        }
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

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
    use crate::domain::error::DomainError;

    #[derive(Debug, PartialEq)]
    struct User {}

    #[test]
    fn test_id() {
        // ok
        assert_eq!(
            Id::<User>::new("01F8MECHZX3TBDSZ7XRADM79XE"),
            Id {
                value: "01F8MECHZX3TBDSZ7XRADM79XE".to_string(),
                _phantom: PhantomData,
            }
        );
    }

    #[test]
    fn test_id_to_string() {
        // ok
        assert_eq!(
            Id::<User>::new("01F8MECHZX3TBDSZ7XRADM79XE").to_string(),
            "01F8MECHZX3TBDSZ7XRADM79XE".to_string()
        );
    }

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
    fn test_name_to_string() {
        // ok
        assert_eq!(
            Name::<User>::new("hoge").unwrap().to_string(),
            "hoge".to_string()
        );
    }
}
