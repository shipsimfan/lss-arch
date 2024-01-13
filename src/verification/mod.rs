use crate::CursesError;
use error::VerificationError;

mod error;

/// Verifies the system is in a correct state to install LSS-Arch
///
/// This function assumes we booted from an Arch Linux install medium
pub fn is_valid_system() -> Result<bool, CursesError> {
    // TODO: Create the verify window

    match verify_system() {
        Ok(()) => Ok(true),
        Err(VerificationError::Curses(error)) => Err(error),
    }
}

/// Actually performs the verification for [`verify_system`]
fn verify_system() -> Result<(), VerificationError> {
    // TODO: Verify running as root

    // TODO: Verify boot mode

    // TODO: Verify internet connection

    Ok(())
}
