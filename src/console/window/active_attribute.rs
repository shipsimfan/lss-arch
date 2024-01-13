use std::ops::{Deref, DerefMut};

use super::Window;
use crate::{console::CursesError, try_curses};
use curses::CHType;

pub struct ActiveAttribute<'a> {
    /// The attribute which was set
    attribute: CHType,

    /// The window the attribute is active on
    window: &'a mut Window,
}

impl<'a> ActiveAttribute<'a> {
    /// Activates `attribute` on `window`
    pub(super) fn new(attribute: CHType, window: &'a mut Window) -> Result<Self, CursesError> {
        try_curses!(curses::wattron(window.inner(), attribute))?;

        Ok(ActiveAttribute { attribute, window })
    }

    /// Deactives the attribute
    pub fn end(self) -> Result<(), CursesError> {
        try_curses!(curses::wattroff(self.window.inner(), self.attribute))
    }
}

impl<'a> Deref for ActiveAttribute<'a> {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        self.window
    }
}

impl<'a> DerefMut for ActiveAttribute<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.window
    }
}

impl<'a> Drop for ActiveAttribute<'a> {
    fn drop(&mut self) {
        unsafe { curses::wattroff(self.window.inner(), self.attribute) };
    }
}
