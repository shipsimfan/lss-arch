use crate::{console::CursesError, error::Error};
use std::fmt::Display;

/// A type that might contain a [`VerificationError`]
pub(super) type VerificationResult<T> = Result<T, VerificationError>;

/// An error while verifying the system state
pub enum VerificationError {
    Curses(CursesError),
}

impl Error for VerificationError {
    fn is_curses_error(&self) -> bool {
        match self {
            VerificationError::Curses(_) => true,
        }
    }

    fn into_curses_error(self) -> Option<CursesError> {
        match self {
            VerificationError::Curses(error) => Some(error),
        }
    }
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::Curses(error) => error.fmt(f),
        }
    }
}

impl From<CursesError> for VerificationError {
    fn from(error: CursesError) -> Self {
        VerificationError::Curses(error)
    }
}
