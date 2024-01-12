use super::VerifyError;
use crate::{print, println, Console};

const BOOT_MODE_PATH: &'static str = "/sys/firmware/efi/fw_platform_size";

/// Verifies the OS was launched from 64-bit UEFI
pub fn verify(console: &mut Console) -> Result<(), VerifyError> {
    print!(console, "  Verifying boot mode . . . ");

    let boot_mode = std::fs::read(BOOT_MODE_PATH).map_err(|_| VerifyError::NotUEFI64Bit)?;
    if boot_mode.as_slice() == b"64\n" {
        Ok(println!(console, "OK"))
    } else {
        Err(VerifyError::NotUEFI64Bit)
    }
}
