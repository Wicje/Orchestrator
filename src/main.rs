mod cli;
mod engine;
mod executor;
mod plan;
mod registry;
mod spec;

fn main() {
    if let Err(e) = cli::Cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
