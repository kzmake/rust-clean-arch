use crate::domain::entity::Entity;
use crate::domain::entity::User;

pub trait AggregateRoot<T>: Entity<T> {}

impl AggregateRoot<User> for User {}
