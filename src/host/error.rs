use super::{ConfigurationError, InstallError};
use crate::{console::CursesError, error::Error};
use std::fmt::Display;

pub(super) type HostInstallResult<T> = Result<T, HostInstallError>;

pub enum HostInstallError {
    Curses(CursesError),
    Configuration(ConfigurationError),
    Install(InstallError),
}

impl Error for HostInstallError {
    fn is_curses_error(&self) -> bool {
        match self {
            HostInstallError::Curses(_) => true,
            HostInstallError::Configuration(_) | HostInstallError::Install(_) => false,
        }
    }

    fn into_curses_error(self) -> Option<CursesError> {
        match self {
            HostInstallError::Curses(error) => Some(error),
            HostInstallError::Configuration(_) | HostInstallError::Install(_) => None,
        }
    }
}

impl Display for HostInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostInstallError::Curses(error) => error.fmt(f),
            HostInstallError::Configuration(error) => error.fmt(f),
            HostInstallError::Install(error) => error.fmt(f),
        }
    }
}

impl From<CursesError> for HostInstallError {
    fn from(error: CursesError) -> Self {
        HostInstallError::Curses(error)
    }
}

impl From<ConfigurationError> for HostInstallError {
    fn from(error: ConfigurationError) -> Self {
        if error.is_curses_error() {
            HostInstallError::Curses(error.into_curses_error().unwrap())
        } else {
            HostInstallError::Configuration(error)
        }
    }
}

impl<T: Into<InstallError>> From<T> for HostInstallError {
    fn from(error: T) -> Self {
        HostInstallError::Install(error.into())
    }
}
