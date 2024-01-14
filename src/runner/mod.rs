use crate::{
    console::{Console, CursesError, MessageWindow},
    host::configure_and_install,
    verification::is_valid_system,
};
use util::try_step;

mod util;

const TITLE: &str = "LSS-Arch Installer";

const WELCOME_MESSAGE: &[&dyn std::fmt::Display] = &[
    &"Welcome to the LSS-Arch Linux installer! This program will",
    &"guide you through the process of installing and setting up",
    &"your new LSS-Arch Linux system.",
];

/// Installs LSS-Arch
pub fn run() -> Result<bool, CursesError> {
    let mut console = Console::new(TITLE)?;

    try_step!(is_valid_system(&mut console), |error| {
        console.colors_mut().enable_error_mode()?;
        MessageWindow::run(&mut console, "Error", &[&error])?;
    });

    MessageWindow::run(&mut console, "Welcome", WELCOME_MESSAGE)?;

    try_step!(configure_and_install(&mut console), |error| {
        console.colors_mut().enable_error_mode()?;
        MessageWindow::run(&mut console, "Error", &[&error])?;
    });

    Ok(true)
}
