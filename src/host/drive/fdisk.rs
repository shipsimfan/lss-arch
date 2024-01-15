use super::InstallError;
use std::{
    io::Write,
    path::Path,
    process::{Child, Command, Stdio},
};

/// The fdisk process
pub(super) struct FDisk(Child);

impl FDisk {
    /// Spawn a new [`FDisk`] process
    pub(super) fn spawn(drive: &Path) -> Result<Self, InstallError> {
        Command::new("fdisk")
            .arg(drive)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::piped())
            .spawn()
            .map(|fdisk| FDisk(fdisk))
            .map_err(InstallError::fdisk_spawn)
    }

    /// Creates a blank GUID partition table on the drive
    pub(super) fn create_guid_table(&mut self) -> Result<(), InstallError> {
        self.write("g\n")
    }

    /// Creates a new partition on the drive
    pub(super) fn create_partition(
        &mut self,
        size: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<(), InstallError> {
        self.write("n\n")?; // Create new partition
        self.write("\n")?; // Default partition number
        self.write("\n")?; // Default start sector

        // Partition size
        if let Some(size) = size {
            self.write(size)?;
        }
        self.write("\n")?;

        if let Some(r#type) = r#type {
            self.write("t\n")?; // Set partition type
            self.write("\n")?; // Default partition number
            self.write(r#type)?;
            self.write("\n")?;
        }

        Ok(())
    }

    /// Writes the new partition table to disk
    pub(super) fn finalize(mut self) -> Result<(), InstallError> {
        self.write("w\nq\n")?;
        if self.0.wait().map_err(InstallError::fdisk_final)?.success() {
            Ok(())
        } else {
            Err(InstallError::FDiskFinalError(None))
        }
    }

    fn write(&mut self, s: &str) -> Result<(), InstallError> {
        self.0
            .stdin
            .as_mut()
            .unwrap()
            .write_all(s.as_bytes())
            .map_err(InstallError::fdisk_write)
    }
}

impl Drop for FDisk {
    fn drop(&mut self) {}
}
