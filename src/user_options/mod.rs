use crate::Console;
use drive::Drive;

mod drive;

/// The installation options the user selected
pub(crate) struct UserOptions {
    drive: Drive,
}

impl UserOptions {
    pub(crate) fn get(console: &mut Console) -> Self {
        let drive = Drive::get(console);

        UserOptions { drive }
    }
}

impl std::fmt::Display for UserOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "The following options will be used for this install:")?;
        write!(f, "  Drive: {}", self.drive)
    }
}
