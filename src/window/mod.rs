use std::{io::Write, ptr::null_mut};

mod error;

pub use error::CursesError;

/// A curses window
pub struct Window;

impl Window {
    /// Creates a new [`Window`]
    pub fn new() -> Result<Self, CursesError> {
        if unsafe { curses::initscr() } == null_mut() {
            return Err(CursesError);
        }

        Ok(Window)
    }

    /// Gets a character from the keyboard
    pub fn get_char(&mut self) -> Result<i32, CursesError> {
        let ret = unsafe { curses::getch() };
        if ret == curses::ERR {
            Err(CursesError)
        } else {
            Ok(ret)
        }
    }
}

impl Write for Window {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if unsafe { curses::addnstr(buf.as_ptr() as _, buf.len() as i32) } == curses::OK {
            Ok(buf.len())
        } else {
            Err(std::io::ErrorKind::Other.into())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if unsafe { curses::refresh() } == curses::OK {
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
