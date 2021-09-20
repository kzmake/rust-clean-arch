use crate::domain::vo::{Id, Name};
use derive_builder::Builder;
use getset::{Getters, Setters};

pub trait Entity<T> {
    fn id(&self) -> Id<T>;
}

#[derive(Default, Clone, Debug, Getters, Setters, Builder)]
#[builder(setter(into))]
pub struct User {
    #[builder(pattern = "immutable")]
    #[getset(get = "pub")]
    id: Id<User>,

    #[getset(get = "pub", set = "pub")]
    name: Name<User>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Entity<User> for User {
    fn id(&self) -> Id<User> {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        assert_eq!(
            UserBuilder::default()
                .id(Id::<User>::new("01F8MECHZX3TBDSZ7XRADM79XE"))
                .name(Name::<User>::new("hoge").unwrap())
                .build()
                .unwrap(),
            User {
                id: Id::<User>::new("01F8MECHZX3TBDSZ7XRADM79XE"),
                name: Name::<User>::new("hoge").unwrap(),
            }
        );
    }
}
