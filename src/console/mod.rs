use colors::Colors;
use error::CursesResult;
use window::Window;

mod colors;
mod curses;
mod error;
mod window;

pub use error::CursesError;

/// A curses instance
pub struct Console {
    /// The root window of the console
    root: curses::Window,

    /// The colors for the application
    colors: Colors,

    /// The width of the console
    width: i32,

    /// The height of the console
    height: i32,
}

/// Sets the basic options in curses for the program
fn set_basic_options(window: curses::Window) -> CursesResult<()> {
    curses::cbreak()?;
    curses::noecho()?;
    curses::keypad(window, true)?;
    curses::curs_set(0)
}

/// Writes the title to the background
fn write_title(window: curses::Window, title: &str) -> CursesResult<()> {
    curses::wattron(window, curses::A_BOLD)?;
    curses::waddnstr(window, title.as_bytes())?;
    curses::wattroff(window, curses::A_BOLD)
}

impl Console {
    /// Creates a new [`Console`]
    pub fn new(title: &str) -> CursesResult<Self> {
        let root = curses::initscr()?;
        let colors = Colors::new()?;

        set_basic_options(root)?;
        curses::wbkgd(root, colors.background_color())?;

        write_title(root, title)?;
        curses::wrefresh(root)?;

        let width = curses::getmaxx(root)?;
        let height = curses::getmaxy(root)?;

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
        curses::touchwin(self.root)?;
        curses::refresh()
    }

    pub(self) fn root(&mut self) -> curses::Window {
        self.root
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        curses::endwin().ok();
    }
}
