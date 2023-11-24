mod errors;
mod fs;
mod view;

use clap::Parser;

use self::{
    errors::Result,
    fs::read_data,
    view::display_data,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Skip `n` bytes
    #[arg(short, long, default_value_t = 0)]
    skip: usize,

    /// Read `n` bytes. None for full read
    #[arg(short, long)]
    length: Option<usize>,

    /// Target file
    filename: String,
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();

    let data = read_data(cli.skip, cli.length, &cli.filename)?;
    display_data(cli.skip, &data)?;

    Ok(())
}
