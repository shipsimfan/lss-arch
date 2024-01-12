use super::VerifyError;
use crate::{print, println, Console};

extern "C" {
    fn geteuid() -> u32;
}

/// Verifies the program is running as "root"
pub fn verify(console: &mut Console) -> Result<(), VerifyError> {
    print!(console, "  Verifying running as root . . . ");

    let euid = unsafe { geteuid() };
    if euid == 0 {
        Ok(println!(console, "OK"))
    } else {
        Err(VerifyError::NotRunningAsRoot)
    }
}
