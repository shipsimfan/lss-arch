use super::step::VerificationStep;
use std::fmt::Display;

pub struct VerifyBootMode;

pub struct NotBootedCorrectly;

impl VerificationStep for VerifyBootMode {
    type Error = NotBootedCorrectly;

    const MESSAGE: &'static str = "Verifying boot mode";

    fn execute() -> Result<(), Self::Error> {
        let value =
            std::fs::read("/sys/firmware/efi/fw_platform_size").map_err(|_| NotBootedCorrectly)?;

        if &value == b"64\n" {
            Ok(())
        } else {
            Err(NotBootedCorrectly)
        }
    }
}

impl Display for NotBootedCorrectly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The system was not booted by UEFI into 64-bit mode")
    }
}
