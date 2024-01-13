use console::{Console, CursesError};
use verification::is_valid_system;

mod console;
mod verification;

const TITLE: &str = "LSS-Arch Installer";

const SUCCESS: i32 = 0;
const FAILURE: i32 = -1;

fn run() -> Result<i32, CursesError> {
    let mut window = Console::new(TITLE)?;

    if !is_valid_system()? {
        return Ok(FAILURE);
    };

    window.get_char()?;
    Ok(SUCCESS)
}

fn main() {
    let exit_code = run()
        .map_err(|error| eprintln!("Error: {}", error))
        .unwrap_or(-1);

    std::process::exit(exit_code);
}
