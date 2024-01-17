use super::step::HostStep;
use crate::error::Error;
use std::fmt::Display;

pub struct Locale;

pub struct LocaleInstallError(std::io::Error);

const LOCALE_CONF: &[u8] = include_bytes!("locale.conf");
const LOCALE_GEN: &[u8] = include_bytes!("locale.gen");

impl HostStep for Locale {
    type ConfigurationError = LocaleInstallError;
    type InstallError = LocaleInstallError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(Locale)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Installing the locale")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        std::fs::write("/mnt/etc/locale.conf", LOCALE_CONF)?;
        std::fs::write("/mnt/etc/locale.gen", LOCALE_GEN)?;
        Ok(())
    }
}

impl Error for LocaleInstallError {}

impl Display for LocaleInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to install the locale - {}", self.0)
    }
}

impl From<std::io::Error> for LocaleInstallError {
    fn from(error: std::io::Error) -> Self {
        LocaleInstallError(error)
    }
}
