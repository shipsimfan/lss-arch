use super::step::HostStep;
use crate::{common::Command, error::Error};
use std::{ffi::CStr, fmt::Display};

pub struct CHRoot;

pub struct CHRootConfigurationError;

pub enum CHRootError {
    ExecutableInstall(std::io::Error),
    CHRoot(std::io::Error),
}

const LSS_ARCH_CHROOT: &[u8] = include_bytes!("chroot.sh");

const LSS_ARCH_CHROOT_PATH: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"/mnt/root/chroot.sh\0") };

impl HostStep for CHRoot {
    type ConfigurationError = CHRootConfigurationError;
    type InstallError = CHRootError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(CHRoot)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Performing chroot actions")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        std::fs::write("/mnt/root/chroot.sh", LSS_ARCH_CHROOT)
            .map_err(|error| CHRootError::ExecutableInstall(error))?;
        unsafe { linux::sys::stat::chmod(LSS_ARCH_CHROOT_PATH.as_ptr(), 0o755) };

        Command::new("arch-chroot")
            .args(["/mnt", "/root/chroot.sh"])
            .run()
            .map_err(|error| CHRootError::CHRoot(error))
    }
}

impl Error for CHRootConfigurationError {}

impl Display for CHRootConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for CHRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CHRootError::ExecutableInstall(error) => {
                write!(f, "Failed to install the chroot program - {}", error)
            }
            CHRootError::CHRoot(error) => write!(f, "Failed to perform chroot tasks - {}", error),
        }
    }
}
