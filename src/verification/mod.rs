use crate::console::Console;
use error::VerificationResult;

mod error;

/// Verifies the system is in a correct state to install LSS-Arch
///
/// This function assumes we booted from an Arch Linux install medium
pub fn is_valid_system(console: &mut Console) -> VerificationResult<()> {
    // TODO: Create the verify progress window
    let mut window = console.new_window(41, 6)?;
    window.flush()?;

    // TODO: Run the verification steps

    window.get_char()?;
    Ok(())
}
