use cli::interface::Cli;

mod cli;
mod handlers;
mod utils;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
