use unox::Cli;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
