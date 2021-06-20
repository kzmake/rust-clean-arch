use crate::vo::{UserId, UserName};
use derive_builder::Builder;
use derive_getters::Getters;

#[derive(Builder, Default, Clone, Debug, Getters)]
#[builder(setter(into))]
pub struct User {
    #[builder(pattern = "immutable")]
    id: UserId,

    name: UserName,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vo::{UserId, UserName};

    #[test]
    fn test_user() {
        assert_eq!(
            UserBuilder::default()
                .id(UserId::new("01F8MECHZX3TBDSZ7XRADM79XE"))
                .name(UserName::new("hoge").unwrap())
                .build()
                .unwrap(),
            User {
                id: UserId::new("01F8MECHZX3TBDSZ7XRADM79XE"),
                name: UserName::new("hoge").unwrap(),
            }
        );
    }
}
