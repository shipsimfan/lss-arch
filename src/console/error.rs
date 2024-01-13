use crate::error::Error;
use std::fmt::Display;

/// A type that may be a [`CursesError`]
pub(super) type CursesResult<T> = Result<T, CursesError>;

/// An error the curses reported
pub struct CursesError;

impl Error for CursesError {
    fn is_curses_error(&self) -> bool {
        true
    }

    fn into_curses_error(self) -> Option<CursesError> {
        Some(self)
    }
}

impl Display for CursesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "curses reported an error")
    }
}

#[macro_export]
macro_rules! try_curses {
    ($expr: expr) => {
        if unsafe { $expr } == ::curses::ERR {
            Err($crate::console::CursesError)
        } else {
            Ok(())
        }
    };
}
