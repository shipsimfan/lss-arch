use crate::try_curses;
use colors::Colors;
use window::Window;

mod colors;
mod error;
mod window;

pub use error::CursesError;

/// A curses instance
pub struct Console {
    /// The root window
    root: Window,

    /// The colors for the application
    colors: Colors,
}

/// Sets the basic options in curses for the program
fn set_basic_options(window: &mut Window) -> Result<(), CursesError> {
    try_curses!(curses::start_color())?;
    try_curses!(curses::cbreak())?;
    try_curses!(curses::noecho())?;
    try_curses!(curses::keypad(window.inner(), true))?;
    try_curses!(curses::curs_set(0))
}

impl Console {
    /// Creates a new [`Window`]
    pub fn new(title: &str) -> Result<Self, CursesError> {
        let mut root = Window::new_root()?;
        set_basic_options(&mut root)?;

        let colors = Colors::new()?;

        root.set_color(colors.background_color())?;
        root.write_with_attribute(title, curses::A_BOLD)?;

        Ok(Console { root, colors })
    }

    /// Gets a character from the keyboard
    pub fn get_char(&mut self) -> Result<i32, CursesError> {
        self.root.get_char()
    }
}
