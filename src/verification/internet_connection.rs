use super::step::VerificationStep;
use crate::common::Command;
use std::fmt::Display;

pub struct VerifyInternetConnection;

pub struct NotConnectedToInternet;

const HOST: &str = "archlinux.org";

impl VerificationStep for VerifyInternetConnection {
    type Error = NotConnectedToInternet;

    const MESSAGE: &'static str = "Verifying internet connection";

    fn execute() -> Result<(), Self::Error> {
        Command::new("ping").args(["-c", "1", HOST]).run()?;

        Ok(())
    }
}

impl Display for NotConnectedToInternet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not ping \"{HOST}\", connect to the internet")
    }
}

impl From<std::io::Error> for NotConnectedToInternet {
    fn from(_: std::io::Error) -> Self {
        NotConnectedToInternet
    }
}
