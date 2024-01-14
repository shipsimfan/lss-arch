use crate::console::{Console, ProgressWindow};
use error::VerificationResult;
use step::{verification_steps, VerificationStep};

mod error;
mod step;

const TITLE: &str = "Verifying System Status";

verification_steps!(
    run_step,
    [
        root::VerifyRoot,
        boot_mode::VerifyBootMode,
        internet_connection::VerifyInternetConnection
    ]
);

/// Verifies the system is in a correct state to install LSS-Arch
///
/// This function assumes we booted from an Arch Linux install medium
pub fn is_valid_system(console: &mut Console) -> VerificationResult<()> {
    let mut window = ProgressWindow::new(console, 3, TITLE)?;

    run_steps(&mut window)?;

    Ok(())
}

fn run_step<Step: VerificationStep>(window: &mut ProgressWindow) -> VerificationResult<()> {
    window.step(Step::MESSAGE)?;

    Step::execute()?;

    window.get_char()?;
    Ok(())
}
