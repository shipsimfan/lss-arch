use crate::console::{Console, ProgressWindow};
use error::VerificationResult;

mod error;

const TITLE: &str = "Verifying System Status";

/// Verifies the system is in a correct state to install LSS-Arch
///
/// This function assumes we booted from an Arch Linux install medium
pub fn is_valid_system(console: &mut Console) -> VerificationResult<()> {
    // TODO: Create the verify progress window
    let mut window = ProgressWindow::new(console, 3, TITLE)?;

    // TODO: Run the verification steps

    window.get_char()?;
    Ok(())
}
