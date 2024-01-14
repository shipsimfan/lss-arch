use crate::console::MessageWindow;
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

    confirm(console, &configuration)?;

    let mut window = ProgressWindow::new(console, configuration.steps(), TITLE)?;
    configuration.install(&mut window)
}

fn confirm(console: &mut Console, configuration: &Configuration) -> HostInstallResult<()> {
    let options = configuration.confirmation();

    let mut lines = vec![
        "These settings will be used for the new LSS-Arch system.".to_owned(),
        "Press ENTER to confirm or CTRL+C to cancel.".to_owned(),
        " ".to_owned(),
    ];

    for (label, value) in options {
        lines.push(format!("{}: {}", label, value));
    }

    MessageWindow::run(console, "Confirm Settings", &lines)?;
    Ok(())
}

fn run_step<Step: HostStep>(window: &mut ProgressWindow, step: Step) -> HostInstallResult<()> {
    window.step(&step.install_message())?;

    step.install()?;

    window.get_char()?;
    Ok(())
}
