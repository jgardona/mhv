use std::fmt::Display;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub static ERR_CANT_PARSE_NUMBER: &str = "Cant parse input number";
pub static ERR_NOT_AVAILABLE_DATA: &str =
    "Cant read data. Check if you are trying to read more than is available, or if the file exists";

#[derive(Debug)]
pub struct CliError {
    pub message: String,
}

impl std::error::Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

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
