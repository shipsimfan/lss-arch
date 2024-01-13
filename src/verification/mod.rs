use error::VerificationResult;

mod error;

/// Verifies the system is in a correct state to install LSS-Arch
///
/// This function assumes we booted from an Arch Linux install medium
pub fn is_valid_system() -> VerificationResult<()> {
    // TODO: Create the verify progress window

    // TODO: Run the verification steps

    Ok(())
}
