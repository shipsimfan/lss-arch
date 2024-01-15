use super::step::HostStep;
use crate::console::{InputWindow, SelectWindow, U8Input};
use configuration_error::ConfigurationError;
use install_error::InstallError;
use std::{
    num::NonZeroU8,
    path::{Path, PathBuf},
};

mod configuration_error;
mod install_error;

pub struct SetupDrive {
    drive: PathBuf,
    swap_size: Option<NonZeroU8>,
}

impl HostStep for SetupDrive {
    type ConfigurationError = ConfigurationError;
    type InstallError = InstallError;

    fn configure(console: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        let mut drives = Vec::new();
        for entry in std::fs::read_dir("/sys/block")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }

            if let Some(drive) = entry.path().file_name() {
                if drive.as_encoded_bytes().starts_with(b"sd") {
                    drives.push(Path::new("/dev").join(drive));
                }
            }
        }

        if drives.len() == 0 {
            return Err(ConfigurationError::NoDrives);
        }

        drives.sort();

        let selected_drive = SelectWindow::run(
            console,
            "Select Drive",
            "Select the drive to install LSS-Arch onto:",
            drives
                .iter()
                .map(|drive| drive.display().to_string())
                .collect(),
        )?;

        let mut swap_size_input = U8Input::new("Swap Size");
        InputWindow::run(
            console,
            "Enter Swap Size",
            "Enter the size of the swap partition, or zero for none",
            &mut [&mut swap_size_input],
        )?;

        let swap_size = NonZeroU8::new(swap_size_input.unwrap().unwrap_or(0));

        Ok(SetupDrive {
            drive: drives.swap_remove(selected_drive),
            swap_size,
        })
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        vec![
            ("Drive", self.drive.display().to_string()),
            (
                "Swap Size",
                if let Some(swap_size) = self.swap_size {
                    format!("{} GB", swap_size)
                } else {
                    "None".to_owned()
                },
            ),
        ]
    }

    fn install_message(&self) -> String {
        format!("Partitioning and mounting {}", self.drive.display())
    }

    fn install(self) -> Result<(), Self::InstallError> {
        Ok(())
    }
}
