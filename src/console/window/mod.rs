use super::{
    curses::{self, CHType},
    Console, CursesResult,
};

mod shadow;

/// A curses window
pub struct Window<'window> {
    /// The underlying curses window
    inner: curses::Window,

    /// The width of the window
    width: i32,

    /// The height of the window
    height: i32,

    /// The x-position of the window
    x: i32,

    /// The y-position of the window
    y: i32,

    /// The console this window is on
    console: &'window mut Console,
}

/// Calculates a centered `(x, y)` position for the window
fn calculate_position(width: i32, height: i32, console: &Console) -> (i32, i32) {
    let x = (console.width() / 2) - (width / 2);
    let y = (console.height() / 2) - (height / 2);
    (x, y)
}

/// Writes the title to the window
fn write_title(window: curses::Window, width: i32, title: &str) -> CursesResult<()> {
    let x = (width / 2) - ((title.len() as i32 + 2) / 2);

    curses::mvwaddch(window, x, 0, ' ')?;
    curses::wattron(window, curses::A_BOLD)?;
    curses::waddnstr(window, title.as_bytes())?;
    curses::wattroff(window, curses::A_BOLD)?;
    curses::waddch(window, ' ')?;

    curses::wrefresh(window)
}

impl<'window> Window<'window> {
    /// Creates a new [`Window`]
    pub(super) fn new(
        console: &'window mut Console,
        width: i32,
        height: i32,
        title: &str,
    ) -> CursesResult<Self> {
        let (x, y) = calculate_position(width, height, console);

        let inner = curses::newwin(x, y, width, height)?;

        curses::wbkgd(inner, console.colors().window_color())?;

        shadow::write(console, x, y, width, height, true)?;
        curses::r#box(inner, 0, 0)?;
        write_title(inner, width, title)?;

        Ok(Window {
            inner,
            width,
            height,
            x,
            y,
            console,
        })
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn console(&self) -> &Console {
        &self.console
    }

    pub fn refresh(&mut self) -> CursesResult<()> {
        curses::wrefresh(self.inner)
    }

    pub fn write_at(&mut self, x: i32, y: i32, s: &[u8]) -> CursesResult<()> {
        curses::mvwaddnstr(self.inner, x, y, s)
    }

    pub fn write_at_with_attribute(
        &mut self,
        x: i32,
        y: i32,
        a: CHType,
        s: &[u8],
    ) -> CursesResult<()> {
        curses::wattron(self.inner, a)?;
        self.write_at(x, y, s)?;
        curses::wattroff(self.inner, a)
    }

    /// Gets a character from the keyboard
    pub fn get_char(&mut self) -> CursesResult<i32> {
        curses::wgetch(self.inner)
    }

    pub(super) fn inner(&mut self) -> curses::Window {
        self.inner
    }
}

impl<'window> Drop for Window<'window> {
    fn drop(&mut self) {
        curses::delwin(self.inner).ok();

        shadow::write(self.console, self.x, self.y, self.width, self.height, false).ok();
        self.console.full_refresh().ok();
    }
}
