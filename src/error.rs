use crate::console::CursesError;
use std::fmt::Display;

/// An error while installing LSS-Arch
pub trait Error: 'static + Display {
    /// Is the error a [`CursesError`]?
    fn is_curses_error(&self) -> bool;

    /// Convert this error to a [`CursesError`]
    fn into_curses_error(self) -> Option<CursesError>;
}
