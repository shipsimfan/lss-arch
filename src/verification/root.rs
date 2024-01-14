use super::step::VerificationStep;
use std::fmt::Display;

pub struct VerifyRoot;

pub struct NotRunningAsRoot;

impl VerificationStep for VerifyRoot {
    type Error = NotRunningAsRoot;

    const MESSAGE: &'static str = "Verifying running as root";

    fn execute() -> Result<(), Self::Error> {
        if unsafe { linux::unistd::geteuid() } == 0 {
            Ok(())
        } else {
            Err(NotRunningAsRoot)
        }
    }
}

impl Display for NotRunningAsRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The program is currently not running as root, please run this program as root"
        )
    }
}
