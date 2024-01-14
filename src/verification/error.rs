use super::NotVerifiedError;
use crate::{console::CursesError, error::Error};
use std::fmt::Display;

/// A type that might contain a [`VerificationError`]
pub(super) type VerificationResult<T> = Result<T, VerificationError>;

/// An error while verifying the system state
pub enum VerificationError {
    Curses(CursesError),
    NotVerified(NotVerifiedError),
}

impl Error for VerificationError {
    fn is_curses_error(&self) -> bool {
        match self {
            VerificationError::Curses(_) => true,
            VerificationError::NotVerified(_) => false,
        }
    }

    fn into_curses_error(self) -> Option<CursesError> {
        match self {
            VerificationError::Curses(error) => Some(error),
            VerificationError::NotVerified(_) => None,
        }
    }
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::Curses(error) => error.fmt(f),
            VerificationError::NotVerified(error) => error.fmt(f),
        }
    }
}

impl From<CursesError> for VerificationError {
    fn from(error: CursesError) -> Self {
        VerificationError::Curses(error)
    }
}

impl<T: Into<NotVerifiedError>> From<T> for VerificationError {
    fn from(error: T) -> Self {
        VerificationError::NotVerified(error.into())
    }
}
