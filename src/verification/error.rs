use crate::CursesError;

/// An error while verifying the system state
pub enum VerificationError {
    Curses(CursesError),
}

impl std::error::Error for VerificationError {}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::Curses(error) => error.fmt(f),
        }
    }
}

impl std::fmt::Debug for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl From<CursesError> for VerificationError {
    fn from(error: CursesError) -> Self {
        VerificationError::Curses(error)
    }
}
