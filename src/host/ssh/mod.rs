use super::step::HostStep;
use crate::error::Error;
use std::fmt::Display;

pub struct SSH;

pub struct SSHInstallError(std::io::Error);

const SSHD_CONFIG: &[u8] = include_bytes!("sshd_config");

const KEYS: &[&[u8]] = &[
    include_bytes!("desktop.pub"),
    include_bytes!("work-laptop.pub"),
];

impl HostStep for SSH {
    type ConfigurationError = SSHInstallError;
    type InstallError = SSHInstallError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(SSH)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Configuring SSH")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        // Write /etc/ssh/sshd_config
        std::fs::write("/mnt/etc/ssh/sshd_config", SSHD_CONFIG)?;

        // Write /root/authorized_keys
        let mut authorized_keys = Vec::new();
        for key in KEYS {
            authorized_keys.extend_from_slice(key);
        }
        std::fs::write("/mnt/root/authorized_keys", authorized_keys)?;

        Ok(())
    }
}

impl Error for SSHInstallError {}

impl Display for SSHInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to install ssh - {}", self.0)
    }
}

impl From<std::io::Error> for SSHInstallError {
    fn from(error: std::io::Error) -> Self {
        SSHInstallError(error)
    }
}
