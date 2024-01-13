use console::{Console, CursesError};

mod console;

fn run() -> Result<(), CursesError> {
    let mut window = Console::new()?;

    window.get_char()?;

    drop(window);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(-1);
        }
    }
}
