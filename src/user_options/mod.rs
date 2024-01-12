use crate::Console;
use drive::Drive;

mod drive;

/// The installation options the user selected
pub(crate) struct UserOptions {
    drive: Drive,
}

impl UserOptions {
    /// Gets the options from the user
    pub(crate) fn get(console: &mut Console) -> Self {
        let drive = Drive::get(console);

        UserOptions { drive }
    }

    /// Gets the drive to install to
    pub(crate) fn drive(&self) -> &Drive {
        &self.drive
    }
}
