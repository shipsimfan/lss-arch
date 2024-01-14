use crate::{console::CursesError, error::Error};
use std::fmt::Display;

pub enum ConfigurationError {
    CursesError(CursesError),
    NoDrives,
    UnableToLoadDrives(std::io::Error),
}

impl Error for ConfigurationError {
    fn is_curses_error(&self) -> bool {
        match self {
            ConfigurationError::CursesError(_) => true,
            ConfigurationError::NoDrives | ConfigurationError::UnableToLoadDrives(_) => false,
        }
    }

    fn into_curses_error(self) -> Option<CursesError> {
        match self {
            ConfigurationError::CursesError(error) => Some(error),
            ConfigurationError::NoDrives | ConfigurationError::UnableToLoadDrives(_) => None,
        }
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::CursesError(error) => error.fmt(f),
            ConfigurationError::NoDrives => write!(f, "No drives found on the system"),
            ConfigurationError::UnableToLoadDrives(error) => {
                write!(f, "Unable to load drives - {}", error.kind())
            }
        }
    }
}

impl From<CursesError> for ConfigurationError {
    fn from(error: CursesError) -> Self {
        ConfigurationError::CursesError(error)
    }
}

impl From<std::io::Error> for ConfigurationError {
    fn from(error: std::io::Error) -> Self {
        ConfigurationError::UnableToLoadDrives(error)
    }
}
