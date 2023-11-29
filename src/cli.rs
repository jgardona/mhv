mod errors;
mod fs;
mod view;

use clap::Parser;

use crate::CliError;

use self::{
    errors::{Result, ERR_CANT_PARSE_NUMBER, ERR_NOT_AVAILABLE_DATA},
    fs::read_data,
    view::display_data,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Skip `N` bytes of the input. The `N` argument can also
    /// include an unit (see `--length` for details).
    #[arg(
        value_name = "N",
        short,
        long,
        default_value = "0",
        verbatim_doc_comment
    )]
    skip: String,

    /// Read `N` bytes from the input. None for full read. The `N`
    /// argument can be a unit with a decimal prefix(kb, mb).
    /// Examples: --length 3kb, -l3kb, --length 1mb...
    /// N unis are kb(1000), K(1024), mb(1000 * 1000) M(1024 * 1024),
    /// and a prefix 0x for hexadecimal, `0x0a`.
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

    let data = read_data(skip, length, &cli.filename)
        .map_err(|_| CliError(ERR_NOT_AVAILABLE_DATA.into()))?;
    display_data(skip, !cli.squeeze, &data, &mut std::io::stdout().lock())?;

    Ok(())
}

fn parse_unit(input: &str) -> std::result::Result<usize, CliError> {
    if let Some(suffix) = input.strip_suffix("kb") {
        let mut value = suffix
            .parse::<usize>()
            .map_err(|_| CliError(ERR_CANT_PARSE_NUMBER.into()))?;
        value *= 1000;
        Ok(value)
    } else if let Some(suffix) = input.strip_suffix("mb") {
        let mut value = suffix
            .parse::<usize>()
            .map_err(|_| CliError(ERR_CANT_PARSE_NUMBER.into()))?;
        value *= 1000 * 1000;
        Ok(value)
    } else if let Some(suffix) = input.strip_suffix('K') {
        let mut value = suffix
            .parse::<usize>()
            .map_err(|_| CliError(ERR_CANT_PARSE_NUMBER.into()))?;
        value *= 1024;
        Ok(value)
    } else if let Some(suffix) = input.strip_suffix('M') {
        let mut value = suffix
            .parse::<usize>()
            .map_err(|_| CliError(ERR_CANT_PARSE_NUMBER.into()))?;
        value *= 1024 * 1024;
        Ok(value)
    } else if let Some(prefix) = input.strip_prefix("0x") {
        let value = usize::from_str_radix(prefix, 16)
            .map_err(|_| CliError(ERR_CANT_PARSE_NUMBER.into()))?;
        Ok(value)
    } else {
        input
            .parse::<usize>()
            .map_err(|_| CliError(ERR_CANT_PARSE_NUMBER.into()))
    }
}

#[cfg(test)]
mod test_cli {
    use crate::{cli::errors::ERR_CANT_PARSE_NUMBER, CliError};

    use super::parse_unit;
    use anyhow::{Ok, Result};

    #[test]
    fn test_parse_unit() -> Result<()> {
        let expected = 1024;
        let result = parse_unit("1K")?;
        assert_eq!(expected, result);

        let expected = 1024 * 1024;
        let result = parse_unit("1M")?;
        assert_eq!(expected, result);

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

        let expected = 10;
        let result = parse_unit("0x0a")?;
        assert_eq!(expected, result);

        let expected = 1024;
        let result = parse_unit("0x400")?;
        assert_eq!(expected, result);

        let expected = CliError(ERR_CANT_PARSE_NUMBER.into());
        let result = parse_unit("fffkb").unwrap_err();
        assert_eq!(expected.to_string(), result.to_string());

        let expected = CliError(ERR_CANT_PARSE_NUMBER.into());
        let result = parse_unit("fffmb").unwrap_err();
        assert_eq!(expected.to_string(), result.to_string());

        let expected = CliError(ERR_CANT_PARSE_NUMBER.into());
        let result = parse_unit("fffK").unwrap_err();
        assert_eq!(expected.to_string(), result.to_string());

        let expected = CliError(ERR_CANT_PARSE_NUMBER.into());
        let result = parse_unit("fffM").unwrap_err();
        assert_eq!(expected.to_string(), result.to_string());

        let expected = CliError(ERR_CANT_PARSE_NUMBER.into());
        let result = parse_unit("0xkkk").unwrap_err();
        assert_eq!(expected.to_string(), result.to_string());

        let expected = CliError(ERR_CANT_PARSE_NUMBER.into());
        let result = parse_unit("ff").unwrap_err();
        assert_eq!(expected.to_string(), result.to_string());

        Ok(())
    }
}
