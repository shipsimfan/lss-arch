use crate::{print, println, Console};
use drive::Drive;
use error::NotConfirmedError;

mod drive;
mod error;

/// The installation options the user selected
pub(crate) struct UserOptions {
    drive: Drive,
}

impl UserOptions {
    pub(crate) fn get(console: &mut Console) -> Result<Self, NotConfirmedError> {
        // Get the options
        let drive = Drive::get(console);

        // Confirm them
        let options = UserOptions { drive };
        options.confirm(console)?;
        Ok(options)
    }

    pub(crate) fn confirm(&self, console: &mut Console) -> Result<(), NotConfirmedError> {
        println!(console);
        println!(
            console,
            "The following options will be used for this install:"
        );
        println!(console, "  Drive: {}", self.drive);

        print!(console, "Do you wish to proceed? [Y/n] ");
        let confirm = console.readln();
        if confirm.as_bytes()[0].to_ascii_lowercase() == b'y' {
            Ok(())
        } else {
            Err(NotConfirmedError)
        }
    }
}
