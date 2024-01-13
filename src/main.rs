use std::io::Write;
use window::{CursesError, Window};

mod window;

fn run() -> Result<(), CursesError> {
    let mut window = Window::new()?;

    writeln!(window, "LSS-Arch v{} installer", env!("CARGO_PKG_VERSION")).unwrap();
    window.flush().unwrap();

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
