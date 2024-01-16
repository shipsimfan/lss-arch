use super::step::HostStep;
use crate::{common::Command, error::Error};
use std::{fmt::Display, io::Write};

pub struct GenFStab;

pub enum GenFStabError {
    FStabFile(std::io::Error),
    GenFStab(std::io::Error),
}

impl HostStep for GenFStab {
    type ConfigurationError = GenFStabError;
    type InstallError = GenFStabError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(GenFStab)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Generating /etc/fstab")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        let output = Command::new("genfstab")
            .args(["-U", "/mnt"])
            .stdout_piped()
            .output()
            .map_err(|error| GenFStabError::GenFStab(error))?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("/mnt/etc/fstab")
            .map_err(|error| GenFStabError::FStabFile(error))?;

        file.write_all(&output.stdout)
            .map_err(|error| GenFStabError::FStabFile(error))
    }
}

impl Error for GenFStabError {}

impl Display for GenFStabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenFStabError::FStabFile(error) => write!(f, "Failed to write /etc/fstab - {}", error),
            GenFStabError::GenFStab(error) => write!(f, "Failed to run \"genfstab\" - {}", error),
        }
    }
}
