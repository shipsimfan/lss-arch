use super::{error::CursesResult, CursesError};
use curses::CHType;
use std::{ffi::c_short, ptr::NonNull};

pub use curses::color_pair;

pub(super) type Window = NonNull<curses::Window>;

macro_rules! try_curses {
    ($expr: expr) => {{
        let result = unsafe { $expr };
        if result == ::curses::ERR {
            Err($crate::console::CursesError)
        } else {
            Ok(result)
        }
    }};
}

pub(super) fn r#box(window: Window, verch: u32, horch: u32) -> CursesResult<()> {
    try_curses!(curses::r#box(window.as_ptr(), verch, horch)).map(|_| ())
}

pub(super) fn can_change_color() -> bool {
    unsafe { curses::can_change_color() }
}

pub(super) fn cbreak() -> CursesResult<()> {
    try_curses!(curses::cbreak()).map(|_| ())
}

pub(super) fn curs_set(visibility: i32) -> CursesResult<()> {
    try_curses!(curses::curs_set(visibility)).map(|_| ())
}

pub(super) fn delwin(window: Window) -> CursesResult<()> {
    try_curses!(curses::delwin(window.as_ptr())).map(|_| ())
}

pub(super) fn derwin(
    parent: Window,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> CursesResult<Window> {
    NonNull::new(unsafe { curses::derwin(parent.as_ptr(), height, width, y, x) }).ok_or(CursesError)
}

pub(super) fn endwin() -> CursesResult<()> {
    try_curses!(curses::endwin()).map(|_| ())
}

pub(super) fn getmaxx(window: Window) -> CursesResult<i32> {
    try_curses!(curses::getmaxx(window.as_ptr()))
}

pub(super) fn getmaxy(window: Window) -> CursesResult<i32> {
    try_curses!(curses::getmaxy(window.as_ptr()))
}

pub(super) fn init_color(color: c_short, r: c_short, g: c_short, b: c_short) -> CursesResult<()> {
    try_curses!(curses::init_color(color, r, g, b)).map(|_| ())
}

pub(super) fn init_pair(pair: c_short, f: c_short, b: c_short) -> CursesResult<()> {
    try_curses!(curses::init_pair(pair, f, b)).map(|_| ())
}

pub(super) fn initscr() -> CursesResult<Window> {
    NonNull::new(unsafe { curses::initscr() }).ok_or(CursesError)
}

pub(super) fn keypad(window: Window, bf: bool) -> CursesResult<()> {
    try_curses!(curses::keypad(window.as_ptr(), bf)).map(|_| ())
}

pub(super) fn mvwaddch(window: Window, x: i32, y: i32, c: char) -> CursesResult<()> {
    try_curses!(curses::mvwaddch(window.as_ptr(), y, x, c as u32)).map(|_| ())
}

pub(super) fn mvwaddnstr(window: Window, x: i32, y: i32, s: &[u8]) -> CursesResult<()> {
    try_curses!(curses::mvwaddnstr(
        window.as_ptr(),
        y,
        x,
        s.as_ptr() as _,
        s.len() as _
    ))
    .map(|_| ())
}

pub(super) fn newwin(x: i32, y: i32, width: i32, height: i32) -> CursesResult<Window> {
    NonNull::new(unsafe { curses::newwin(height, width, y, x) }).ok_or(CursesError)
}

pub(super) fn noecho() -> CursesResult<()> {
    try_curses!(curses::noecho()).map(|_| ())
}

pub(super) fn refresh() -> CursesResult<()> {
    try_curses!(curses::refresh()).map(|_| ())
}

pub(super) fn start_color() -> CursesResult<()> {
    try_curses!(curses::start_color()).map(|_| ())
}

pub(super) fn touchwin(window: Window) -> CursesResult<()> {
    try_curses!(curses::touchwin(window.as_ptr())).map(|_| ())
}

pub(super) fn waddch(window: Window, c: char) -> CursesResult<()> {
    try_curses!(curses::waddch(window.as_ptr(), c as u32)).map(|_| ())
}

pub(super) fn waddnstr(window: Window, s: &[u8]) -> CursesResult<()> {
    try_curses!(curses::waddnstr(
        window.as_ptr(),
        s.as_ptr() as _,
        s.len() as _
    ))
    .map(|_| ())
}

pub(super) fn wattroff(window: Window, attribute: CHType) -> CursesResult<()> {
    try_curses!(curses::wattroff(window.as_ptr(), attribute)).map(|_| ())
}

pub(super) fn wattron(window: Window, attribute: CHType) -> CursesResult<()> {
    try_curses!(curses::wattron(window.as_ptr(), attribute)).map(|_| ())
}

pub(super) fn wbkgd(window: Window, ch: CHType) -> CursesResult<()> {
    try_curses!(curses::wbkgd(window.as_ptr(), ch | b' ' as CHType)).map(|_| ())
}

pub(super) fn wgetch(window: Window) -> CursesResult<i32> {
    try_curses!(curses::wgetch(window.as_ptr()))
}

pub(super) fn wmove(window: Window, x: i32, y: i32) -> CursesResult<()> {
    try_curses!(curses::wmove(window.as_ptr(), y, x)).map(|_| ())
}

pub(super) fn wrefresh(window: Window) -> CursesResult<()> {
    try_curses!(curses::wrefresh(window.as_ptr())).map(|_| ())
}
