use crate::console::CursesError;
use std::{convert::Infallible, fmt::Display};

/// An error while installing LSS-Arch
pub trait Error: 'static + Display + Sized {
    /// Is the error a [`CursesError`]?
    fn is_curses_error(&self) -> bool {
        false
    }

    /// Convert this error to a [`CursesError`]
    fn into_curses_error(self) -> Option<CursesError> {
        None
    }
}

impl Error for Infallible {}
