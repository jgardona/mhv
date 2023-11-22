pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod errors_test {
    use std::{error::Error, fmt::Display};

    use crate::cli::errors::Result;

    #[derive(Debug)]
    struct TestError;
    impl Error for TestError {}
    impl Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Simple error")
        }
    }

    #[test]
    fn test_result() {
        fn return_error() -> Result<()> {
            Err(Box::new(TestError))
        }
        assert!(return_error().is_err());
    }
}
