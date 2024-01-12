use crate::Drive;
use std::{
    io::Write,
    process::{Child, Command, Stdio},
};

/// The fdisk process
pub(super) struct FDisk(Child);

impl FDisk {
    /// Spawn a new [`FDisk`] process
    pub(super) fn spawn(drive: &Drive) -> std::io::Result<Self> {
        let fdisk = Command::new("fdisk")
            .arg(drive)
            .stdout(Stdio::null())
            .stdin(Stdio::piped())
            .spawn()?;

        Ok(FDisk(fdisk))
    }

    pub(super) fn create_guid_table(&mut self) -> std::io::Result<()> {
        self.write("g\n")
    }

    pub(super) fn create_partition(
        &mut self,
        size: Option<&str>,
        r#type: Option<&str>,
    ) -> std::io::Result<()> {
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

    pub(super) fn finalize(mut self) -> std::io::Result<()> {
        self.write("w\nq\n")
    }

    fn write(&mut self, s: &str) -> std::io::Result<()> {
        self.0.stdin.as_mut().unwrap().write_all(s.as_bytes())
    }
}

impl Drop for FDisk {
    fn drop(&mut self) {
        self.0.wait().unwrap();
    }
}
