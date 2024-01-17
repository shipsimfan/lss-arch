use super::step::HostStep;
use crate::{
    console::{Console, CursesError, SelectWindow},
    error::Error,
};
use r#static::StaticIP;
use std::fmt::Display;

mod r#static;

pub enum Internet {
    DHCP,
    Static(StaticIP),
}

pub enum InternetConfigurationError {
    Curses(CursesError),
    NetDevices(std::io::Error),
}

pub struct InternetInstallError(std::io::Error);

impl HostStep for Internet {
    type ConfigurationError = InternetConfigurationError;
    type InstallError = InternetInstallError;

    fn configure(console: &mut Console) -> Result<Self, Self::ConfigurationError> {
        let mut interfaces = Vec::new();
        for entry in std::fs::read_dir("/sys/class/net")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }

            if let Some(interface) = entry.path().file_name() {
                if interface.as_encoded_bytes().starts_with(b"e") {
                    interfaces.push(interface.to_string_lossy().to_string());
                }
            }
        }

        interfaces.sort();

        let mut options = Vec::with_capacity(interfaces.len() + 1);
        options.push("DHCP".to_owned());
        options.extend(interfaces.clone());

        Ok(
            match SelectWindow::run(
                console,
                "Interface",
                "Select the network interface to be used",
                options,
            )
            .map_err(|error| InternetConfigurationError::Curses(error))?
            {
                0 => Internet::DHCP,
                i => Internet::Static(StaticIP::configure(interfaces.swap_remove(i - 1), console)?),
            },
        )
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        match self {
            Internet::DHCP => vec![("IP", "DHCP".to_owned())],
            Internet::Static(static_ip) => static_ip.confirm(),
        }
    }

    fn install_message(&self) -> String {
        format!("Setting internet settings")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        match self {
            Internet::DHCP => {
                // The chroot will see the "/root/set-ip.sh" missing and enable the DHCPCD service instead
                Ok(())
            }
            Internet::Static(r#static) => r#static
                .install()
                .map_err(|error| InternetInstallError(error)),
        }
    }
}

impl Error for InternetConfigurationError {
    fn is_curses_error(&self) -> bool {
        match self {
            InternetConfigurationError::Curses(_) => true,
            InternetConfigurationError::NetDevices(_) => false,
        }
    }

    fn into_curses_error(self) -> Option<CursesError> {
        match self {
            InternetConfigurationError::Curses(error) => Some(error),
            InternetConfigurationError::NetDevices(_) => None,
        }
    }
}

impl Display for InternetConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternetConfigurationError::Curses(error) => error.fmt(f),
            InternetConfigurationError::NetDevices(error) => {
                write!(f, "Failed to list network interfaces - {}", error)
            }
        }
    }
}

impl From<CursesError> for InternetConfigurationError {
    fn from(error: CursesError) -> Self {
        InternetConfigurationError::Curses(error)
    }
}

impl From<std::io::Error> for InternetConfigurationError {
    fn from(error: std::io::Error) -> Self {
        InternetConfigurationError::NetDevices(error)
    }
}

impl Display for InternetInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to set internet settings - {}", self.0)
    }
}
