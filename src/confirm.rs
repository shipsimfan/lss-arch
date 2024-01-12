use crate::{print, println, Console, UserOptions};

/// The user input was not confirmed
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct NotConfirmedError;

/// Gets the user to confirm the provided options
pub(crate) fn confirm(
    options: &UserOptions,
    console: &mut Console,
) -> Result<(), NotConfirmedError> {
    println!(
        console,
        "The following options will be used for this install:"
    );
    println!(console, "  Drive: {}", options.drive());
    println!(console, "  Swap Size: {}", options.swap_size());
    println!(console, "  Username: {}", options.username());
    println!(console, "  TimeZone: {}", options.time_zone());

    print!(console, "Do you wish to proceed? [Y/n] ");
    let confirm = console.readln();
    if confirm.len() > 0 && confirm.as_bytes()[0].to_ascii_lowercase() == b'y' {
        Ok(())
    } else {
        Err(NotConfirmedError)
    }
}

impl std::error::Error for NotConfirmedError {}

impl std::fmt::Display for NotConfirmedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "install options not confirmed, aborting")
    }
}

impl std::fmt::Debug for NotConfirmedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
