use super::CursesResult;
use crate::try_curses;
use curses::{CHType, COLOR_BLACK, COLOR_BLUE, COLOR_WHITE, COLOR_YELLOW};
use std::ffi::c_short;

/// Color manager
pub struct Colors;

// Predefined color pair indices
const BACKGROUND_COLOR_PAIR: c_short = 1; // Blue back + yellow fore
const NORMAL_COLOR_PAIR: c_short = 2;

// Attributes for color pairs
const A_BACKGROUND_COLOR_PAIR: CHType = curses::color_pair!(BACKGROUND_COLOR_PAIR);
const A_NORMAL_COLOR_PAIR: CHType = curses::color_pair!(NORMAL_COLOR_PAIR);

/// Initializes all colors used
fn init_colors() -> CursesResult<()> {
    if !unsafe { curses::can_change_color() } {
        return Ok(());
    }

    init_color(COLOR_BLACK, [0, 0, 0])?;
    init_color(COLOR_BLUE, [0, 0, 656])?;
    init_color(COLOR_YELLOW, [968, 984, 312])?;
    init_color(COLOR_WHITE, [656, 656, 656])
}

/// Initializes all the color pairs
fn init_color_pairs() -> CursesResult<()> {
    init_color_pair(BACKGROUND_COLOR_PAIR, COLOR_YELLOW, COLOR_BLUE)?;
    init_color_pair(NORMAL_COLOR_PAIR, COLOR_BLACK, COLOR_WHITE)
}

/// Initializes `color` to `rgb`
fn init_color(color: c_short, rgb: [c_short; 3]) -> CursesResult<()> {
    try_curses!(curses::init_color(color, rgb[0], rgb[1], rgb[2],))
}

/// Initializes a color pair
fn init_color_pair(pair: c_short, f: c_short, b: c_short) -> CursesResult<()> {
    try_curses!(curses::init_pair(pair, f, b))
}

impl Colors {
    /// Initializes the colors used by the system
    pub fn new() -> CursesResult<Self> {
        try_curses!(curses::start_color())?;

        init_colors()?;
        init_color_pairs()?;

        Ok(Colors)
    }

    /// The color used for the background window
    pub fn background_color(&self) -> CHType {
        A_BACKGROUND_COLOR_PAIR
    }

    /// The color used for windows
    pub fn window_color(&self) -> CHType {
        A_NORMAL_COLOR_PAIR
    }
}
