use crate::try_curses;
use colors::Colors;
use error::CursesResult;
use init::init;
use std::ptr::NonNull;
use window::Window;

mod colors;
mod error;
mod init;
mod window;

pub use error::CursesError;

/// A curses instance
pub struct Console {
    /// The root window of the console
    root: NonNull<curses::Window>,

    /// The colors for the application
    colors: Colors,

    /// The width of the console
    width: i32,

    /// The height of the console
    height: i32,
}

impl Console {
    /// Creates a new [`Console`]
    pub fn new(title: &str) -> CursesResult<Self> {
        let (root, colors, width, height) = init(title)?;

        Ok(Console {
            root,
            colors,
            width,
            height,
        })
    }

    /// Gets the [`Colors`] for the program
    pub fn colors(&self) -> &Colors {
        &self.colors
    }

    /// Gets the width of the console
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Gets the height of the console
    pub fn height(&self) -> i32 {
        self.height
    }

    // Creates a new [`Window`] on the console
    pub fn new_window(&mut self, width: i32, height: i32, title: &str) -> CursesResult<Window> {
        Window::new(self, width, height, title)
    }

    pub(self) fn full_refresh(&mut self) -> CursesResult<()> {
        try_curses!(curses::touchwin(self.root.as_ptr()))?;
        try_curses!(curses::refresh())?;
        Ok(())
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}
