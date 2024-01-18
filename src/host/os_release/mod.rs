use super::step::HostStep;
use crate::error::Error;
use std::fmt::Display;

pub struct OSRelease;

pub struct OSReleaseInstallError(std::io::Error);

const ISSUE: &[u8] = include_bytes!("issue");
const LSB_RELEASE: &[u8] = include_bytes!("lsb-release");
const OS_RELEASE: &[u8] = include_bytes!("os-release");

impl HostStep for OSRelease {
    type ConfigurationError = OSReleaseInstallError;
    type InstallError = OSReleaseInstallError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(OSRelease)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Installing the distribution info")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        std::fs::write("/mnt/etc/issue", ISSUE).map_err(|error| OSReleaseInstallError(error))?;
        std::fs::write("/mnt/etc/lsb-release", LSB_RELEASE)
            .map_err(|error| OSReleaseInstallError(error))?;
        std::fs::write("/mnt/etc/os-release", OS_RELEASE)
            .map_err(|error| OSReleaseInstallError(error))
    }
}

impl Error for OSReleaseInstallError {}

impl Display for OSReleaseInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to install the os release - {}", self.0)
    }
}
