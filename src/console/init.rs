use super::{Colors, CursesError, CursesResult};
use crate::try_curses;
use std::ptr::NonNull;

/// Initializes curses
pub(super) fn init(title: &str) -> CursesResult<(Colors, i32, i32)> {
    let root_window = start_curses()?;
    let colors = Colors::new()?;

    set_basic_options(root_window)?;
    set_background_color(&colors)?;
    write_title(title)?;
    flush()?;

    let width = get_width(root_window)?;
    let height = get_height(root_window)?;

    Ok((colors, width, height))
}

/// Starts curses
fn start_curses() -> CursesResult<NonNull<curses::Window>> {
    NonNull::new(unsafe { curses::initscr() }).ok_or(CursesError)
}

/// Gets the width of the console
fn get_width(window: NonNull<curses::Window>) -> CursesResult<i32> {
    try_curses!(curses::getmaxx(window.as_ptr()))
}

/// Gets the height of the console
fn get_height(window: NonNull<curses::Window>) -> CursesResult<i32> {
    try_curses!(curses::getmaxy(window.as_ptr()))
}

/// Sets the basic options in curses for the program
fn set_basic_options(window: NonNull<curses::Window>) -> CursesResult<()> {
    try_curses!(curses::cbreak())?;
    try_curses!(curses::noecho())?;
    try_curses!(curses::keypad(window.as_ptr(), true))?;
    try_curses!(curses::curs_set(0))?;
    Ok(())
}

/// Sets the background color
fn set_background_color(colors: &Colors) -> CursesResult<()> {
    try_curses!(curses::bkgd(colors.background_color()))?;
    Ok(())
}

/// Writes the title to the background
fn write_title(title: &str) -> CursesResult<()> {
    try_curses!(curses::attron(curses::A_BOLD))?;
    try_curses!(curses::addnstr(title.as_ptr() as _, title.len() as _))?;
    try_curses!(curses::attroff(curses::A_BOLD))?;
    Ok(())
}

/// Flushes the changes to disk
fn flush() -> CursesResult<()> {
    try_curses!(curses::refresh())?;
    Ok(())
}
