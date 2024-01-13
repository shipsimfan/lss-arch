use crate::{
    console::{colors::Colors, error::CursesResult, Console, CursesError},
    try_curses,
};
use curses::CHType;
use std::ptr::NonNull;

/// Calculates a centered `(x, y)` position for the window
pub(super) fn calculate_position(width: i32, height: i32, console: &Console) -> (i32, i32) {
    let x = (console.width() / 2) - (width / 2);
    let y = (console.height() / 2) - (height / 2);
    (x, y)
}

/// Creates a [`curses::Window`] at `(x, y)` sized `width` by `height`
pub(super) fn create_window(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: CHType,
) -> CursesResult<NonNull<curses::Window>> {
    let window = NonNull::new(unsafe { curses::newwin(height, width, y, x) }).ok_or(CursesError)?;
    try_curses!(curses::wbkgd(window.as_ptr(), color | b' ' as CHType)).map(|_| window)
}

/// Writes the shadow behind a window
pub(super) fn write_shadow(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    colors: Option<&Colors>,
) -> CursesResult<()> {
    if let Some(colors) = colors {
        try_curses!(curses::attron(colors.shadow_color()))?;
    }

    try_curses!(curses::mvaddnstr(y + height, x + 1, " ".as_ptr() as _, 1))?;
    for _ in 1..width {
        try_curses!(curses::addnstr(" ".as_ptr() as _, 1))?;
    }

    for iy in 0..height - 1 {
        try_curses!(curses::mvaddnstr(
            y + 1 + iy,
            x + width,
            " ".as_ptr() as _,
            1
        ))?;
    }

    if let Some(colors) = colors {
        try_curses!(curses::attroff(colors.shadow_color()))?;
    }
    try_curses!(curses::refresh())?;
    Ok(())
}

/// Writes the border around the window
pub(super) fn write_border(inner: NonNull<curses::Window>) -> CursesResult<()> {
    try_curses!(curses::r#box(inner.as_ptr(), 0, 0))?;
    Ok(())
}

/// Writes the title to the window
pub(super) fn write_title(
    inner: NonNull<curses::Window>,
    width: i32,
    title: &str,
) -> CursesResult<()> {
    let x = (width / 2) - ((title.len() as i32 + 2) / 2);

    try_curses!(curses::mvwaddnstr(
        inner.as_ptr(),
        0,
        x,
        " ".as_ptr() as _,
        1
    ))?;
    try_curses!(curses::wattron(inner.as_ptr(), curses::A_BOLD))?;
    try_curses!(curses::waddnstr(
        inner.as_ptr(),
        title.as_ptr() as _,
        title.len() as _
    ))?;
    try_curses!(curses::wattroff(inner.as_ptr(), curses::A_BOLD))?;
    try_curses!(curses::waddnstr(inner.as_ptr(), " ".as_ptr() as _, 1))?;

    try_curses!(curses::wrefresh(inner.as_ptr()))?;
    Ok(())
}
