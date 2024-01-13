use crate::{try_curses, CursesError};
use curses::CHType;
use std::ptr::null_mut;

/// A curses window
pub struct Window {
    /// The underlying curses window
    inner: *mut curses::Window,
}

pub struct ActiveAttribute(CHType);

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
    pub fn set_color(&mut self, color: CHType) -> Result<(), CursesError> {
        try_curses!(curses::wbkgd(self.inner, color))
    }

    /// Set an attribute for future writes
    pub fn set_attribute(&mut self, attribute: CHType) -> Result<ActiveAttribute, CursesError> {
        try_curses!(curses::wattron(self.inner, attribute)).map(|_| ActiveAttribute(attribute))
    }

    /// Writes `s` to the window
    pub fn write(&mut self, s: &str) -> Result<(), CursesError> {
        try_curses!(curses::waddnstr(
            self.inner,
            s.as_ptr() as _,
            s.len() as i32
        ))?;
        try_curses!(curses::wrefresh(self.inner))
    }

    /// Gets a character from the keyboard
    #[allow(unused_unsafe)]
    pub fn get_char(&mut self) -> Result<i32, CursesError> {
        let ret = unsafe { curses::wgetch(self.inner) };
        try_curses!(ret).map(|_| ret)
    }

    /// Gets the underlying curses window
    pub(super) unsafe fn inner(&mut self) -> *mut curses::Window {
        self.inner
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}

impl Drop for ActiveAttribute {
    fn drop(&mut self) {
        unsafe { curses::attroff(self.0) };
    }
}
