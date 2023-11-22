mod errors;
mod fs;
mod view;

use clap::Parser;

use self::{errors::Result, fs::read_data, view::display_data};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Skip `n` offset(1 = 16) bytes
    #[arg(short, long, default_value_t = 0)]
    skip: usize,

    /// Only read `n` offset bytes from the input. Skip for full read
    #[arg(short, long)]
    length: Option<usize>,

    /// Target file
    filename: String,
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    let data = read_data(&cli.filename)?;
    let length = if let Some(len) = cli.length {
        len
    } else {
        data.len()
    };
    display_data(cli.skip, length, &data);
    Ok(())
}
