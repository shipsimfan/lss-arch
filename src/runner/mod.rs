use crate::{
    console::{Console, CursesError},
    verification::is_valid_system,
};
use util::try_step;

mod util;

const TITLE: &str = "LSS-Arch Installer";

/// Installs LSS-Arch
pub fn run() -> Result<bool, CursesError> {
    let mut console = Console::new(TITLE)?;

    try_step!(is_valid_system(&mut console), |error| todo!(
        "Handle error ({})",
        error
    ));

    unsafe { curses::refresh() };
    unsafe { curses::getch() };

    Ok(true)
}
