use crate::console::{curses, error::CursesResult, Console};

/// Writes the shadow behind a window
pub(super) fn write(
    console: &mut Console,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    create: bool,
) -> CursesResult<()> {
    let window = console.root();

    if create {
        curses::wattron(window, console.colors().shadow_color())?;
    }

    // Horizontal shadow
    curses::wmove(window, x + 1, y + height)?;
    for _ in 0..width {
        curses::waddch(window, ' ')?;
    }

    // Vertical shadow
    for iy in 0..height - 1 {
        curses::mvwaddch(window, x + width, y + 1 + iy, ' ')?;
    }

    if create {
        curses::wattroff(window, console.colors().shadow_color())?;
    }

    curses::wrefresh(window)
}
