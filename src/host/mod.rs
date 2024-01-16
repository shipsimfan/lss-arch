use crate::console::{Console, MessageWindow, ProgressWindow};
use error::HostInstallResult;
use step::{steps, HostStep};

mod error;
mod step;

const TITLE: &str = "Installing LSS-Arch";

steps!(
    run_step,
    [
        drive::SetupDrive,
        packages::InstallPackages,
        fstab::GenFStab
    ]
);

pub fn configure_and_install(console: &mut Console) -> HostInstallResult<()> {
    let configuration = Configuration::get(console)?;

    confirm(console, &configuration)?;

    let mut window = ProgressWindow::new(console, configuration.steps(), TITLE)?;
    configuration.install(&mut window)
}

fn confirm(console: &mut Console, configuration: &Configuration) -> HostInstallResult<()> {
    const MESSAGE_LINES: &[&str] = &[
        "These settings will be used for the new LSS-Arch system.",
        "Press ENTER to confirm or CTRL+C to cancel.",
        " ",
    ];

    let options = configuration.confirmation();

    let mut width = MESSAGE_LINES[0].len();
    for (label, option) in &options {
        let half_width = label.len().max(option.len()) + 6;
        width = width.max(half_width * 2);
    }

    let mut lines = Vec::with_capacity(MESSAGE_LINES.len() + options.len());
    for line in MESSAGE_LINES {
        if line.len() == 0 {
            lines.push(String::new());
            continue;
        }

        let mut string = String::with_capacity((width - line.len()) / 2 + line.len());
        for _ in 0..(width - line.len()) / 2 {
            string.push(' ');
        }
        string.push_str(line);
        lines.push(string);
    }

    for (label, option) in options {
        let mut string = String::with_capacity((width / 2) + option.len() + 1);
        for _ in 0..(width / 2) - label.len() - 1 {
            string.push(' ');
        }
        string.push_str(label);
        string.push_str(": ");
        string.push_str(&option);
        lines.push(string);
    }

    MessageWindow::run(console, "Confirm Settings", &lines)?;
    Ok(())
}

fn run_step<Step: HostStep>(window: &mut ProgressWindow, step: Step) -> HostInstallResult<()> {
    window.step(&step.install_message())?;

    step.install()?;

    Ok(())
}
