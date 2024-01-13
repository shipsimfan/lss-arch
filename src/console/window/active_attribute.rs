use super::Window;
use crate::{console::CursesError, try_curses};
use curses::CHType;

pub struct ActiveAttribute<'a> {
    /// The attribute which was set
    attribute: CHType,

    /// The window the attribute is active on
    window: &'a Window,
}

impl<'a> ActiveAttribute<'a> {
    /// Activates `attribute` on `window`
    pub(super) fn new(attribute: CHType, window: &'a Window) -> Result<Self, CursesError> {
        try_curses!(curses::wattron(window.inner(), attribute))?;

        Ok(ActiveAttribute { attribute, window })
    }

    /// Deactives the attribute
    pub fn end(self) -> Result<(), CursesError> {
        try_curses!(curses::wattroff(self.window.inner(), self.attribute))
    }
}

impl<'a> Drop for ActiveAttribute<'a> {
    fn drop(&mut self) {
        unsafe { curses::wattroff(self.window.inner(), self.attribute) };
    }
}
