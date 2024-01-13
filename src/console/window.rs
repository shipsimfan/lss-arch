use crate::{try_curses, CursesError};
use curses::CHType;
use std::ptr::null_mut;

/// A curses window
pub struct Window {
    /// The underlying curses window
    inner: *mut curses::Window,
}

impl Window {
    /// Create the root window and initialize curses
    pub(super) fn new_root() -> Result<Self, CursesError> {
        let inner = unsafe { curses::initscr() };
        if inner == null_mut() {
            return Err(CursesError);
        } else {
            Ok(Window { inner })
        }
    }

    /// Sets the foreground and background color of the window
    pub fn set_color(&self, color: CHType) -> Result<(), CursesError> {
        try_curses!(curses::wbkgd(self.inner, color | b' ' as CHType))
    }

    /// Writes `s` to the window
    pub fn write(&self, s: &str) -> Result<(), CursesError> {
        try_curses!(curses::waddnstr(
            self.inner,
            s.as_ptr() as _,
            s.len() as i32
        ))?;
        try_curses!(curses::wrefresh(self.inner))
    }

    /// Writes `s` to the window with `attribute`
    pub fn write_with_attribute(&self, s: &str, attribute: CHType) -> Result<(), CursesError> {
        try_curses!(curses::wattron(self.inner, attribute))?;
        self.write(s)?;
        try_curses!(curses::wattroff(self.inner, attribute))
    }

    /// Gets a character from the keyboard
    #[allow(unused_unsafe)]
    pub fn get_char(&self) -> Result<i32, CursesError> {
        let ret = unsafe { curses::wgetch(self.inner) };
        try_curses!(ret).map(|_| ret)
    }

    /// Gets the underlying curses window
    pub(super) unsafe fn inner(&self) -> *mut curses::Window {
        self.inner
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}
