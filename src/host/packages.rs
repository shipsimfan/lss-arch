use super::step::HostStep;
use crate::{common::Command, error::Error};
use std::fmt::Display;

pub struct InstallPackages;

pub struct InstallPackagesError(std::io::Error);

pub const PACKAGES: &[&str] = &[
    // Core packages
    "base", "linux",
];

impl HostStep for InstallPackages {
    type ConfigurationError = InstallPackagesError;
    type InstallError = InstallPackagesError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(InstallPackages)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Installing packages (this may take a while)")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        // Update the keyring
        Command::new("pacman")
            .args(["-Sy", "--noconfirm", "archlinux-keyring"])
            .run()
            .map_err(|error| InstallPackagesError(error))?;

        // Install the packages
        Command::new("pacstrap")
            .args(["-K", "/mnt"])
            .args(PACKAGES)
            .run()
            .map_err(|error| InstallPackagesError(error))
    }
}

impl Error for InstallPackagesError {}

impl Display for InstallPackagesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to install packages - {}", self.0)
    }
}
