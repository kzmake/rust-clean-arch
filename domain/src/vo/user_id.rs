#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id() {
        // ok
        assert_eq!(
            UserId::new("01F8MECHZX3TBDSZ7XRADM79XE"),
            UserId("01F8MECHZX3TBDSZ7XRADM79XE".to_string())
        );
    }
}
