use cli::execute;

mod cli;

fn main() {
    execute().unwrap_or_else(|e| {
        eprintln!("Something wrong happened: {e}");
        std::process::exit(1);
    });
}
