use crate::try_curses;
use std::{io::Write, ptr::null_mut};

mod error;

pub use error::CursesError;

/// A curses window
pub struct Window {
    /// The underlying curses window
    inner: *mut curses::Window,
}

impl Window {
    /// Creates a new [`Window`]
    pub fn new() -> Result<Self, CursesError> {
        // Create the window
        let inner = unsafe { curses::initscr() };
        if inner == null_mut() {
            return Err(CursesError);
        }

        // Setup basic options
        try_curses!(unsafe { curses::cbreak() })?;
        try_curses!(unsafe { curses::noecho() })?;
        try_curses!(unsafe { curses::keypad(inner, true) })?;

        Ok(Window { inner })
    }

    /// Gets a character from the keyboard
    pub fn get_char(&mut self) -> Result<i32, CursesError> {
        let ret = unsafe { curses::wgetch(self.inner) };
        if ret == curses::ERR {
            Err(CursesError)
        } else {
            Ok(ret)
        }
    }
}

impl Write for Window {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if unsafe { curses::waddnstr(self.inner, buf.as_ptr() as _, buf.len() as i32) }
            == curses::OK
        {
            Ok(buf.len())
        } else {
            Err(std::io::ErrorKind::Other.into())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if unsafe { curses::wrefresh(self.inner) } == curses::OK {
            Ok(())
        } else {
            Err(std::io::ErrorKind::Other.into())
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}
