use crate::try_curses;
use std::ptr::null_mut;

mod error;

pub use error::CursesError;

/// A curses instance
pub struct Console {
    /// The underlying curses window
    screen: *mut curses::Window,
}

impl Console {
    /// Creates a new [`Window`]
    pub fn new() -> Result<Self, CursesError> {
        // Create the screen and initialize curses
        let screen = unsafe { curses::initscr() };
        if screen == null_mut() {
            return Err(CursesError);
        }

        // Setup basic input options
        try_curses!(unsafe { curses::cbreak() })?;
        try_curses!(unsafe { curses::noecho() })?;
        try_curses!(unsafe { curses::keypad(screen, true) })?;

        Ok(Console { screen })
    }

    /// Gets a character from the keyboard
    pub fn get_char(&mut self) -> Result<i32, CursesError> {
        let ret = unsafe { curses::wgetch(self.screen) };
        if ret == curses::ERR {
            Err(CursesError)
        } else {
            Ok(ret)
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}
