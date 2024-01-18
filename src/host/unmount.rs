use super::step::HostStep;
use crate::{common::Command, error::Error};
use std::fmt::Display;

pub struct Unmount;

pub struct UnmountError(std::io::Error);

impl HostStep for Unmount {
    type ConfigurationError = UnmountError;
    type InstallError = UnmountError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(Unmount)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Unmounting the drive")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        Command::new("umount").args(["-R", "/mnt"]).run()?;
        Ok(())
    }
}

impl Error for UnmountError {}

impl Display for UnmountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to unmount the drive - {}", self.0)
    }
}

impl From<std::io::Error> for UnmountError {
    fn from(error: std::io::Error) -> Self {
        UnmountError(error)
    }
}
