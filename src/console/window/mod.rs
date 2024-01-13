use super::{Console, CursesError, CursesResult};
use crate::try_curses;
use curses::CHType;
use std::ptr::NonNull;

mod init;

/// A curses window
pub struct Window<'window> {
    /// The underlying curses window
    inner: NonNull<curses::Window>,

    /// The width of the window
    width: i32,

    /// The height of the window
    height: i32,

    /// The console this window is on
    console: &'window mut Console,
}

impl<'window> Window<'window> {
    /// Creates a new [`Window`]
    pub(super) fn new(
        console: &'window mut Console,
        width: i32,
        height: i32,
        title: &str,
    ) -> CursesResult<Self> {
        let (x, y) = init::calculate_position(width, height, console);

        let inner = init::create_window(x, y, width, height, console.colors().window_color())?;

        // TODO: Add the shadow

        init::write_border(inner)?;
        init::write_title(inner, width, title)?;

        Ok(Window {
            inner,
            width,
            height,
            console,
        })
    }

    /// Gets a character from the keyboard
    pub fn get_char(&mut self) -> CursesResult<i32> {
        try_curses!(curses::wgetch(self.inner.as_ptr()))
    }
}

impl<'window> Drop for Window<'window> {
    fn drop(&mut self) {
        unsafe { curses::delwin(self.inner.as_ptr()) };

        // TODO: Erase the shadow
    }
}
