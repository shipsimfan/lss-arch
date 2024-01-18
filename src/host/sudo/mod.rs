use super::step::HostStep;
use crate::error::Error;
use std::fmt::Display;

pub struct Sudo;

pub struct SudoInstallError(std::io::Error);

const SUDOERS: &[u8] = include_bytes!("sudoers");

impl HostStep for Sudo {
    type ConfigurationError = SudoInstallError;
    type InstallError = SudoInstallError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(Sudo)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Installing sudo")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        std::fs::write("/mnt/etc/sudoers", SUDOERS)?;
        Ok(())
    }
}

impl Error for SudoInstallError {}

impl Display for SudoInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to install sudo - {}", self.0)
    }
}

impl From<std::io::Error> for SudoInstallError {
    fn from(error: std::io::Error) -> Self {
        SudoInstallError(error)
    }
}
