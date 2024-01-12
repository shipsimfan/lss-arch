use super::{println, Console};
use error::VerifyError;

mod error;

/// Verifies the program is running in a valid state before installation
pub(super) fn verify_state(console: &mut Console) -> Result<(), VerifyError> {
    println!(console, "Verifying the system state . . .");

    // Check for root

    // Check for boot mode

    // Check for internet

    Ok(())
}
