use colors::Colors;
use error::CursesResult;
use init::init;
use window::Window;

mod colors;
mod error;
mod init;
mod window;

pub use error::CursesError;

/// A curses instance
pub struct Console {
    /// The colors for the application
    colors: Colors,

    /// The width of the terminal
    width: i32,

    /// The height of the terminal
    height: i32,
}

impl Console {
    /// Creates a new [`Console`]
    pub fn new(title: &str) -> CursesResult<Self> {
        let (colors, width, height) = init(title)?;

        Ok(Console {
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
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}
