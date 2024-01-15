use super::step::HostStep;
use crate::{
    common::Command,
    console::{InputWindow, SelectWindow, U8Input},
};
use configuration_error::ConfigurationError;
use fdisk::FDisk;
use install_error::InstallError;
use std::{
    num::NonZeroU8,
    path::{Path, PathBuf},
};

mod configuration_error;
mod fdisk;
mod install_error;

pub struct SetupDrive {
    drive: PathBuf,
    swap_size: Option<NonZeroU8>,
}

impl SetupDrive {
    fn partition_name(&self, index: usize) -> PathBuf {
        self.drive.with_file_name(format!(
            "{}{}",
            self.drive.file_name().unwrap().to_string_lossy(),
            index
        ))
    }
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
        // Unmount to be safe
        Command::new("umount").args(["-R", "/mnt"]).run().ok();
        Command::new("swapoff").arg("-a").run().ok();

        // Get the parition names
        let boot_partition = self.partition_name(1);
        let swap_partition = self.swap_size.map(|_| self.partition_name(2));
        let root_partition = self.partition_name(self.swap_size.map(|_| 3).unwrap_or(2));

        // Parition the drive
        let mut fdisk = FDisk::spawn(&self.drive)?;
        fdisk.create_guid_table()?;
        fdisk.create_partition(Some("+1G"), Some("1"))?; // Boot partition
        self.swap_size
            .map(|swap_size| fdisk.create_partition(Some(&format!("+{}G", swap_size)), Some("19")))
            .unwrap_or(Ok(()))?; // Swap partition
        fdisk.create_partition(None, None)?; // Root partition
        fdisk.finalize()?;

        // Format the partitions
        Command::new("mkfs.vfat")
            .args(["-F", "32"])
            .arg(&boot_partition)
            .run()
            .map_err(InstallError::boot_format)?;

        if let Some(swap_partition) = &swap_partition {
            Command::new("mkswap")
                .arg(&swap_partition)
                .run()
                .map_err(InstallError::swap_format)?;
        }

        Command::new("mkfs.ext4")
            .arg(&root_partition)
            .run()
            .map_err(InstallError::root_format)?;

        // Mount the partitions
        Command::new("mount")
            .arg(root_partition)
            .arg("/mnt")
            .run()
            .map_err(InstallError::root_mount)?;

        Command::new("mount")
            .arg(boot_partition)
            .args(["/mnt/boot", "--mkdir"])
            .run()
            .map_err(InstallError::boot_mount)?;

        if let Some(swap_partition) = swap_partition {
            Command::new("swapon")
                .arg(swap_partition)
                .run()
                .map_err(InstallError::swap_mount)?;
        }

        Ok(())
    }
}
