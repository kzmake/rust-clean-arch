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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_to_string() {
        // ok
        assert_eq!(
            Id::<User>::new("01F8MECHZX3TBDSZ7XRADM79XE").to_string(),
            "01F8MECHZX3TBDSZ7XRADM79XE".to_string()
        );
    }
}
