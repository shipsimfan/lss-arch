mod console;
mod error;
mod runner;
mod verification;

fn main() {
    let success = runner::run()
        .map_err(|error| eprintln!("Error: {}", error))
        .unwrap_or(false);

    std::process::exit(if success { 0 } else { -1 });
}
