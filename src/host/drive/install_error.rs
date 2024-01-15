use std::fmt::Display;

pub enum InstallError {
    FDiskSpawnError(std::io::Error),
    FDiskWriteError(std::io::Error),
    FDiskFinalError(Option<std::io::Error>),
    BootFormatError(std::io::Error),
    SwapFormatError(std::io::Error),
    RootFormatError(std::io::Error),
    BootMountError(std::io::Error),
    SwapMountError(std::io::Error),
    RootMountError(std::io::Error),
}

impl InstallError {
    pub(super) fn fdisk_spawn(error: std::io::Error) -> Self {
        InstallError::FDiskSpawnError(error)
    }

    pub(super) fn fdisk_write(error: std::io::Error) -> Self {
        InstallError::FDiskWriteError(error)
    }

    pub(super) fn fdisk_final(error: std::io::Error) -> Self {
        InstallError::FDiskFinalError(Some(error))
    }

    pub(super) fn boot_format(error: std::io::Error) -> Self {
        InstallError::BootFormatError(error)
    }

    pub(super) fn swap_format(error: std::io::Error) -> Self {
        InstallError::SwapFormatError(error)
    }

    pub(super) fn root_format(error: std::io::Error) -> Self {
        InstallError::RootFormatError(error)
    }

    pub(super) fn boot_mount(error: std::io::Error) -> Self {
        InstallError::BootMountError(error)
    }

    pub(super) fn swap_mount(error: std::io::Error) -> Self {
        InstallError::SwapMountError(error)
    }

    pub(super) fn root_mount(error: std::io::Error) -> Self {
        InstallError::RootMountError(error)
    }
}

impl Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallError::FDiskSpawnError(error) => {
                write!(f, "Could not open fdisk - {}", error.kind())
            }
            InstallError::FDiskWriteError(error) => {
                write!(f, "An error occured while using fdisk - {}", error.kind())
            }
            InstallError::FDiskFinalError(error) => {
                write!(f, "fdisk failed to write the partitions")?;
                if let Some(error) = error {
                    write!(f, " - {}", error)?;
                }
                Ok(())
            }
            InstallError::BootFormatError(error) => {
                write!(f, "Unable to format the boot partition - {}", error)
            }
            InstallError::SwapFormatError(error) => {
                write!(f, "Unable to format the swap partition - {}", error)
            }
            InstallError::RootFormatError(error) => {
                write!(f, "Unable to format the root partition - {}", error)
            }
            InstallError::BootMountError(error) => {
                write!(f, "Unable to mount the boot partition - {}", error)
            }
            InstallError::SwapMountError(error) => {
                write!(f, "Unable to mount the swap partition - {}", error)
            }
            InstallError::RootMountError(error) => {
                write!(f, "Unable to mount the root partition - {}", error)
            }
        }
    }
}
