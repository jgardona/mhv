use std::{error::Error, fmt::Display};

use cli::execute;

mod cli;

#[derive(Debug)]
pub struct CliError(String);

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    execute().unwrap_or_else(|e| {
        eprintln!("Something wrong happened: {e}");
        std::process::exit(1);
    });
}
