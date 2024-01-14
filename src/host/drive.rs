use super::step::HostStep;
use crate::error::Error;
use std::fmt::Display;

pub struct SetupDrive {}

pub struct DriveError;

pub struct ConfigurationError;

impl HostStep for SetupDrive {
    type ConfigurationError = ConfigurationError;
    type InstallError = DriveError;

    fn configure(console: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(SetupDrive {})
    }

    fn install_message(&self) -> String {
        format!("Partitioning and mounting X")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        Ok(())
    }
}

impl Display for DriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ConfigurationError {}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
