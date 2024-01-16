use crate::{
    console::{Console, CursesError, MessageWindow},
    host::configure_and_install,
    verification::is_valid_system,
};
use util::try_step;

mod util;

const TITLE: &str = "LSS-Arch Installer";

/// Installs LSS-Arch
pub fn run() -> Result<bool, CursesError> {
    let mut console = Console::new(TITLE)?;

    try_step!(is_valid_system(&mut console), |error| {
        console.colors_mut().enable_error_mode()?;
        MessageWindow::run(&mut console, "Error", &[error.to_string()])?;
    });

    MessageWindow::run(
        &mut console,
        "Welcome",
        &[
            "Welcome to the LSS-Arch Linux installer! This program will".to_owned(),
            "guide you through the process of installing and setting up".to_owned(),
            "your new LSS-Arch Linux system.".to_owned(),
        ],
    )?;

    try_step!(configure_and_install(&mut console), |error| {
        console.colors_mut().enable_error_mode()?;
        MessageWindow::run(&mut console, "Error", &[error.to_string()])?;
    });

    MessageWindow::run(
        &mut console,
        "Install Complete",
        &["The installation has finished! You may now reboot your computer".to_owned()],
    )
    .map(|_| true)
}
