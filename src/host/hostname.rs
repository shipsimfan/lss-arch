use super::step::HostStep;
use crate::{
    console::{Console, CursesError, InputWindow, StringInput},
    error::Error,
};
use std::fmt::Display;

pub struct Hostname(String);

pub struct HostnameConfigurationError(CursesError);

pub struct HostnameInstallError(std::io::Error);

impl HostStep for Hostname {
    type ConfigurationError = HostnameConfigurationError;
    type InstallError = HostnameInstallError;

    fn configure(console: &mut Console) -> Result<Self, Self::ConfigurationError> {
        let mut hostname_input = StringInput::new("Hostname", 24);
        InputWindow::run(
            console,
            "Set Hostname",
            "Enter a hostname for the new system. Defaults to \"unnamed\"",
            &mut [&mut hostname_input],
        )
        .map_err(|error| HostnameConfigurationError(error))?;

        let mut hostname = hostname_input.unwrap().trim().to_owned();
        if hostname.len() == 0 {
            hostname = "unnamed".to_owned();
        }
        hostname.push('\n');

        Ok(Hostname(hostname))
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        vec![("Hostname", self.0.trim().to_owned())]
    }

    fn install_message(&self) -> String {
        format!("Setting hostname")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        std::fs::write("/mnt/etc/hostname", self.0).map_err(|error| HostnameInstallError(error))
    }
}

impl Error for HostnameConfigurationError {
    fn is_curses_error(&self) -> bool {
        true
    }

    fn into_curses_error(self) -> Option<CursesError> {
        Some(self.0)
    }
}

impl Display for HostnameConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to configure hostname - {}", self.0)
    }
}

impl Display for HostnameInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to install the hostname - {}", self.0)
    }
}
