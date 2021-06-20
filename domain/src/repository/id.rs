use anyhow::Error;

pub trait IdRepository {
    fn generate(&self) -> Result<String, Error>;
}

#[cfg(test)]
mod tests {
    use crate::IdRepository;
    use anyhow::Error;

    const ID: &str = "01F8MECHZX3TBDSZ7XRADM79XE";

    struct MockIdRepository {}
    impl MockIdRepository {
        fn new() -> Self {
            Self {}
        }
    }
    impl IdRepository for MockIdRepository {
        fn generate(&self) -> Result<String, Error> {
            Ok(ID.to_string())
        }
    }

    #[test]
    fn test_generate() {
        let sut: &dyn IdRepository = &MockIdRepository::new();
        // ok
        assert_eq!(sut.generate().unwrap(), ID.to_string());
    }
}
