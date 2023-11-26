mod errors;
mod fs;
mod view;

use std::num::ParseIntError;

use clap::Parser;

use self::{errors::Result, fs::read_data, view::display_data};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Skip `N` bytes of the input. The `N` argument can also
    /// include an unit (see `--length` for details).
    #[arg(value_name = "N", short, long, default_value = "0", verbatim_doc_comment)]
    skip: String,

    /// Read `N` bytes from the input. None for full read. The `N`
    /// argument can be a unit with a decimal prefix(kb, mb).
    /// Examples: --length 3kb, -l3kb, --length 1mb...
    #[arg(value_name = "N", short, long, verbatim_doc_comment)]
    length: Option<String>,

    /// Displays all input data. Otherwise any number of output
    /// lines which would be identical to the last one are replaced
    /// with a line comprised of a single asterisk.
    #[arg(short = 'n', long = "no-squeezing", verbatim_doc_comment)]
    squeeze: bool,

    /// Target file.
    filename: String,
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    let length = parse_unit(&cli.length.unwrap_or("0".into()))?;
    let skip = parse_unit(&cli.skip)?;

    let data = read_data(skip, length, &cli.filename)?;
    display_data(skip, !cli.squeeze, &data)?;

    Ok(())
}

fn parse_unit(input: &str) -> std::result::Result<usize, ParseIntError> {
    if input.ends_with("kb") {
        let mut value = input[..input.len() - 2].parse::<usize>()?;
        value *= 1000;
        Ok(value)
    } else if input.ends_with("mb") {
        let mut value = input[..input.len() - 2].parse::<usize>()?;
        value *= 1000000;
        Ok(value)
    } else {
        input.parse::<usize>()
    }
}

#[cfg(test)]
mod test_cli {
    use anyhow::{Ok, Result};

    use super::parse_unit;
    #[test]
    fn test_parse_unit() -> Result<()> {
        let expected = 3000;
        let result = parse_unit("3kb")?;
        assert_eq!(expected, result);

        let expected = 1000;
        let result = parse_unit("1kb")?;
        assert_eq!(expected, result);

        let expected = 2;
        let result = parse_unit("2")?;
        assert_eq!(expected, result);

        let expected = 2000000;
        let result = parse_unit("2mb")?;
        assert_eq!(expected, result);

        let expected = 1000000;
        let result = parse_unit("1mb")?;
        assert_eq!(expected, result);

        Ok(())
    }
}
