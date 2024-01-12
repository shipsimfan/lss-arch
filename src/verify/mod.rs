use super::{println, Console};
use error::VerifyError;

mod boot_mode;
mod error;
mod internet;
mod root;

/// Verifies the program is running in a valid state before installation
pub(super) fn verify_state(console: &mut Console) -> Result<(), VerifyError> {
    println!(console, "Verifying the system state . . .");

    root::verify(console)?;
    boot_mode::verify(console)?;
    internet::verify(console)?;

    Ok(())
}
