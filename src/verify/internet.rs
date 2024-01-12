use super::VerifyError;
use crate::{print, println, Console};
use std::process::{Command, Stdio};

const PING_ARGS: &[&str] = &["-c", "1", "archlinux.org"];

/// Verifies the program is connected to the internet
pub fn verify(console: &mut Console) -> Result<(), VerifyError> {
    print!(console, "  Verifying internet connection . . . ");

    let ping = Command::new("ping")
        .args(PING_ARGS)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| VerifyError::NotConnectedToInternet)?;

    if ping.success() {
        Ok(println!(console, "OK"))
    } else {
        Err(VerifyError::NotConnectedToInternet)
    }
}
