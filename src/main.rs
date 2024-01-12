use confirm::confirm;
use console::{print, println, prompt, Console};
use format::format;
use mount::mount;
use partition::partition;
use user_options::{Drive, Internet, SwapSize, UserOptions};

mod confirm;
mod console;
mod format;
mod mount;
mod partition;
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

    println!(
        console,
        "Installing LSS Arch to {} . . .",
        user_options.drive()
    );
    partition(console, user_options.drive(), user_options.swap_size())?;
    format(console, user_options.drive(), user_options.swap_size())?;
    mount(console, user_options.drive(), user_options.swap_size())?;

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
