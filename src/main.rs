use console::{print, println, prompt, Console};
use user_options::UserOptions;

use crate::confirm::confirm;

mod confirm;
mod console;
mod user_options;
mod verify;

fn run(console: &mut Console) -> Result<(), Box<dyn std::error::Error>> {
    println!(console, "Preparing to install LSS Arch . . .");

    verify::verify_state(console)?;
    println!(console);

    let user_options = UserOptions::get(console);
    println!(console);

    confirm(&user_options, console)?;
    println!(console);

    Ok(())
}

fn main() {
    let mut console = Console::new();

    match run(&mut console) {
        Ok(()) => {}
        Err(error) => {
            println!(console, "Error: {}", error);
            std::process::exit(-1);
        }
    }
}
