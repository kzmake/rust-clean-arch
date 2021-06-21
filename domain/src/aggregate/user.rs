use derive_builder::Builder;
use getset::{Getters, Setters};

use crate::vo::{Id, Name};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vo::{Id, Name};

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
