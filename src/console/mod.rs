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
    pub fn new(title: &str) -> Result<Self, CursesError> {
        // Create the screen and initialize curses
        let screen = unsafe { curses::initscr() };
        if screen == null_mut() {
            return Err(CursesError);
        }

        // Setup basic input options
        try_curses!(unsafe { curses::cbreak() })?;
        try_curses!(unsafe { curses::noecho() })?;
        try_curses!(unsafe { curses::keypad(screen, true) })?;

        // Disable the cursor
        try_curses!(unsafe { curses::curs_set(0) })?;

        // TODO: Set background to dark blue

        // TODO: Set foreground to yellow

        // Write the title
        try_curses!(unsafe { curses::waddnstr(screen, title.as_ptr() as _, title.len() as _) })?;
        try_curses!(unsafe { curses::wrefresh(screen) })?;

        Ok(Console { screen })
    }

    /// Gets a character from the keyboard
    pub fn get_char(&mut self) -> Result<i32, CursesError> {
        let ret = unsafe { curses::wgetch(self.screen) };
        try_curses!(ret).map(|_| ret)
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}
