use crate::console::{Console, ProgressWindow};
use error::HostInstallResult;
use step::steps;
use step::HostStep;

mod error;
mod step;

const TITLE: &str = "Installing LSS-Arch";

steps!(run_step, [drive::SetupDrive]);

pub fn configure_and_install(console: &mut Console) -> HostInstallResult<()> {
    let configuration = Configuration::get(console)?;

    let mut window = ProgressWindow::new(console, configuration.steps(), TITLE)?;
    configuration.install(&mut window)
}

fn run_step<Step: HostStep>(window: &mut ProgressWindow, step: Step) -> HostInstallResult<()> {
    window.step(&step.install_message())?;

    step.install()?;

    window.get_char()?;
    Ok(())
}
