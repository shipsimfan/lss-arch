use std::ops::{Deref, DerefMut};

use super::Window;
use crate::{console::CursesError, try_curses};
use curses::CHType;

pub struct ActiveAttribute<'attribute, 'window> {
    /// The attribute which was set
    attribute: CHType,

    /// The window the attribute is active on
    window: &'attribute mut Window<'window>,
}

impl<'attribute, 'window> ActiveAttribute<'attribute, 'window> {
    /// Activates `attribute` on `window`
    pub(super) fn new(
        attribute: CHType,
        window: &'attribute mut Window<'window>,
    ) -> Result<Self, CursesError> {
        try_curses!(curses::wattron(window.inner(), attribute))?;

        Ok(ActiveAttribute { attribute, window })
    }

    /// Deactives the attribute
    pub fn end(self) -> Result<(), CursesError> {
        try_curses!(curses::wattroff(self.window.inner(), self.attribute))
    }
}

impl<'attribute, 'window> Deref for ActiveAttribute<'attribute, 'window> {
    type Target = Window<'window>;

    fn deref(&self) -> &Self::Target {
        self.window
    }
}

impl<'attribute, 'window> DerefMut for ActiveAttribute<'attribute, 'window> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.window
    }
}

impl<'attribute, 'window> Drop for ActiveAttribute<'attribute, 'window> {
    fn drop(&mut self) {
        unsafe { curses::wattroff(self.window.inner(), self.attribute) };
    }
}
