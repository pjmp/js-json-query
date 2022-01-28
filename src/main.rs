use std::process;

mod cli;

fn main() {
    if let Err(err) = cli::App::run() {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
