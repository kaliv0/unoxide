mod cli;
mod handlers;
mod utils;

use cli::Cli;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
