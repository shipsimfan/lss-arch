use crate::Console;
use drive::Drive;
use swap_size::SwapSize;

mod drive;
mod swap_size;

/// The installation options the user selected
pub(crate) struct UserOptions {
    drive: Drive,
    swap_size: SwapSize,
}

impl UserOptions {
    /// Gets the options from the user
    pub(crate) fn get(console: &mut Console) -> Self {
        let drive = Drive::get(console);
        let swap_size = SwapSize::get(console);

        UserOptions { drive, swap_size }
    }

    /// Gets the drive to install to
    pub(crate) fn drive(&self) -> &Drive {
        &self.drive
    }

    /// Gets the size of the swap partition
    pub(crate) fn swap_size(&self) -> SwapSize {
        self.swap_size
    }
}
