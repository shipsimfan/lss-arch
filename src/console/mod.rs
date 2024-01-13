use crate::try_curses;
use colors::Colors;
use error::CursesResult;
use window::Window;

mod colors;
mod error;
mod window;

pub use error::CursesError;

/// A curses instance
pub struct Console<'a> {
    /// The root window
    root: Window<'a>,

    /// The colors for the application
    colors: Colors,
}

/// Sets the basic options in curses for the program
fn set_basic_options(window: &mut Window) -> CursesResult<()> {
    try_curses!(curses::start_color())?;
    try_curses!(curses::cbreak())?;
    try_curses!(curses::noecho())?;
    try_curses!(curses::keypad(window.inner(), true))?;
    try_curses!(curses::curs_set(0))
}

impl<'a> Console<'a> {
    /// Creates a new [`Window`]
    pub fn new(title: &str) -> CursesResult<Self> {
        let mut root = Window::new_root()?;
        set_basic_options(&mut root)?;

        let colors = Colors::new()?;

        root.set_color(colors.background_color())?;
        root.write_with_attribute(title, curses::A_BOLD)?;

        Ok(Console { root, colors })
    }

    /// Creates a new [`Window`] on the console
    pub fn new_window(&mut self, width: i32, height: i32) -> CursesResult<Window> {
        let x = (self.root.width() / 2) - (width / 2);
        let y = (self.root.height() / 2) - (height / 2);

        let mut window = self.root.subwindow(x, y, width, height)?;
        window.set_color(self.colors.window_color())?;
        Ok(window)
    }
}
