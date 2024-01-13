use super::{CursesError, CursesResult};
use crate::try_curses;
use active_attribute::ActiveAttribute;
use curses::CHType;
use std::{marker::PhantomData, ptr::null_mut};

mod active_attribute;

/// A curses window
pub struct Window<'window> {
    /// The underlying curses window
    inner: *mut curses::Window,

    /// Represents the parent window
    parent: PhantomData<&'window mut ()>,
}

impl<'window> Window<'window> {
    /// Create the root window and initialize curses
    pub(super) fn new_root() -> CursesResult<Self> {
        let inner = unsafe { curses::initscr() };
        if inner == null_mut() {
            return Err(CursesError);
        } else {
            Ok(Window {
                inner,
                parent: PhantomData,
            })
        }
    }

    /// Gets the width of the window
    pub fn width(&self) -> i32 {
        unsafe { curses::getmaxx(self.inner) }
    }

    /// Gets the height of the window
    pub fn height(&self) -> i32 {
        unsafe { curses::getmaxy(self.inner) }
    }

    /// Creates a sub-window to this window
    pub fn subwindow<'child>(
        &'child mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> CursesResult<Window<'child>> {
        let inner = unsafe { curses::subwin(self.inner, height, width, y, x) };
        if inner == null_mut() {
            return Err(CursesError);
        } else {
            Ok(Window {
                inner,
                parent: PhantomData,
            })
        }
    }

    /// Sets the foreground and background color of the window
    pub fn set_color(&mut self, color: CHType) -> CursesResult<()> {
        try_curses!(curses::wbkgd(self.inner, color | b' ' as CHType))
    }

    /// Sets an attribute for future writes
    pub fn set_attribute<'attribute>(
        &'attribute mut self,
        attribute: CHType,
    ) -> CursesResult<ActiveAttribute<'attribute, 'window>> {
        ActiveAttribute::new(attribute, self)
    }

    /// Writes `s` to the window
    pub fn write(&mut self, s: &str) -> CursesResult<()> {
        try_curses!(curses::waddnstr(
            self.inner,
            s.as_ptr() as _,
            s.len() as i32
        ))?;
        try_curses!(curses::wrefresh(self.inner))
    }

    /// Writes `s` to the window with `attribute`
    pub fn write_with_attribute(&mut self, s: &str, attribute: CHType) -> CursesResult<()> {
        let mut active_attribute = self.set_attribute(attribute)?;
        active_attribute.write(s)?;
        active_attribute.end()
    }

    /// Gets a character from the keyboard
    #[allow(unused_unsafe)]
    pub fn get_char(&mut self) -> CursesResult<i32> {
        let ret = unsafe { curses::wgetch(self.inner) };
        try_curses!(ret).map(|_| ret)
    }

    /// Gets the underlying curses window
    pub(super) unsafe fn inner(&mut self) -> *mut curses::Window {
        self.inner
    }
}

impl<'window> Drop for Window<'window> {
    fn drop(&mut self) {
        unsafe { curses::endwin() };
    }
}