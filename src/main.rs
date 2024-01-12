use console::{print, println, Console};

mod console;
mod verify;

fn run(console: &mut Console) -> Result<(), Box<dyn std::error::Error>> {
    println!(console, "Preparing to install LSS Arch . . .");

    verify::verify_state(console)?;

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
