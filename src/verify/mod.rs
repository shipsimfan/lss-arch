use super::{println, Console};
use error::VerifyError;

mod error;
mod root;

/// Verifies the program is running in a valid state before installation
pub(super) fn verify_state(console: &mut Console) -> Result<(), VerifyError> {
    println!(console, "Verifying the system state . . .");

    root::verify(console)?;

    // Check for boot mode

    // Check for internet

    Ok(())
}
