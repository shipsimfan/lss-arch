use crate::{
    console::{Console, CursesError},
    verification::is_valid_system,
};
use util::try_step;

mod util;

const TITLE: &str = "LSS-Arch Installer";

/// Installs LSS-Arch
pub fn run() -> Result<bool, CursesError> {
    let mut window = Console::new(TITLE)?;

    try_step!(is_valid_system(), |error| todo!("Handle error ({})", error));

    window.get_char()?;
    Ok(true)
}
