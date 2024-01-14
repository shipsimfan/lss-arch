use super::{curses, CursesResult};
use curses::{CHType, COLOR_BLACK, COLOR_BLUE, COLOR_RED, COLOR_WHITE, COLOR_YELLOW};
use std::ffi::c_short;

/// Color manager
pub struct Colors;

// Predefined color pair indices
const BACKGROUND_COLOR_PAIR: c_short = 1; // Blue back + yellow fore
const NORMAL_COLOR_PAIR: c_short = 2; // Gray back + black fore
const SHADOW_COLOR_PAIR: c_short = 3; // Black back + black fore

/// Initializes all colors used
fn init_colors() -> CursesResult<()> {
    if !curses::can_change_color() {
        return Ok(());
    }

    curses::init_color(COLOR_BLACK, 0, 0, 0)?;
    curses::init_color(COLOR_BLUE, 0, 0, 656)?;
    curses::init_color(COLOR_YELLOW, 968, 984, 312)?;
    curses::init_color(COLOR_WHITE, 656, 656, 656)?;
    curses::init_color(COLOR_RED, 656, 0, 0)
}

/// Initializes all the color pairs
fn init_color_pairs() -> CursesResult<()> {
    curses::init_pair(BACKGROUND_COLOR_PAIR, COLOR_YELLOW, COLOR_BLUE)?;
    curses::init_pair(NORMAL_COLOR_PAIR, COLOR_BLACK, COLOR_WHITE)?;
    curses::init_pair(SHADOW_COLOR_PAIR, COLOR_BLACK, COLOR_BLACK)
}

impl Colors {
    /// Initializes the colors used by the system
    pub fn new() -> CursesResult<Self> {
        curses::start_color()?;

        init_colors()?;
        init_color_pairs()?;

        Ok(Colors)
    }

    /// The color used for the background window
    pub fn background_color(&self) -> CHType {
        curses::color_pair!(BACKGROUND_COLOR_PAIR)
    }

    /// The color used for windows
    pub fn window_color(&self) -> CHType {
        curses::color_pair!(NORMAL_COLOR_PAIR)
    }

    /// The color used for window shadows
    pub fn shadow_color(&self) -> CHType {
        curses::color_pair!(SHADOW_COLOR_PAIR)
    }

    pub fn enable_error_mode(&mut self) -> CursesResult<()> {
        curses::init_pair(BACKGROUND_COLOR_PAIR, COLOR_WHITE, COLOR_RED)
    }
}
