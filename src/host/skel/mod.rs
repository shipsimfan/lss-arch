use super::step::HostStep;
use crate::error::Error;
use std::fmt::Display;

pub struct UserSkeleton;

pub struct UserSkeletonInstallError(std::io::Error);

const BASHRC: &[u8] = include_bytes!(".bashrc");
const VIMRC: &[u8] = include_bytes!(".vimrc");

impl HostStep for UserSkeleton {
    type ConfigurationError = UserSkeletonInstallError;
    type InstallError = UserSkeletonInstallError;

    fn configure(_: &mut crate::console::Console) -> Result<Self, Self::ConfigurationError> {
        Ok(UserSkeleton)
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        Vec::new()
    }

    fn install_message(&self) -> String {
        format!("Installing the user skeleton")
    }

    fn install(self) -> Result<(), Self::InstallError> {
        std::fs::write("/mnt/etc/skel/.bashrc", BASHRC)?;
        std::fs::write("/mnt/etc/skel/.vimrc", VIMRC)?;
        Ok(())
    }
}

impl Error for UserSkeletonInstallError {}

impl Display for UserSkeletonInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to install the user skeleton - {}", self.0)
    }
}

impl From<std::io::Error> for UserSkeletonInstallError {
    fn from(error: std::io::Error) -> Self {
        UserSkeletonInstallError(error)
    }
}
