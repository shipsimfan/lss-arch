use super::step::HostStep;
use crate::error::Error;
use std::fmt::Display;

pub struct NFTables;

pub struct NFTablesInstallError(std::io::Error);

const NFTABLES_CONF: &[u8] = include_bytes!("nftables.conf");

impl HostStep for NFTables {
    type ConfigurationError = NFTablesInstallError;
    type InstallError = NFTablesInstallError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(NFTables)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Installing the firewall configuration")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        std::fs::write("/mnt/etc/nftables.conf", NFTABLES_CONF)?;
        Ok(())
    }
}

impl Error for NFTablesInstallError {}

impl Display for NFTablesInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to install the firewall configuration - {}",
            self.0
        )
    }
}

impl From<std::io::Error> for NFTablesInstallError {
    fn from(error: std::io::Error) -> Self {
        NFTablesInstallError(error)
    }
}
