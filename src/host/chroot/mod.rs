use super::step::HostStep;
use crate::{
    common::Command,
    console::{Console, CursesError, InputWindow, StringInput},
    error::Error,
};
use std::{ffi::CStr, fmt::Display};

pub struct CHRoot {
    time_zone: String,
    username: String,
}

pub struct CHRootConfigurationError(CursesError);

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

    fn configure(console: &mut Console) -> Result<Self, Self::ConfigurationError> {
        let mut time_zone = StringInput::new("Time Zone", 24);
        InputWindow::run(
            console,
            "Set Time Zone",
            "Enter the time zone. Defaults to \"America/Toronto\"",
            &mut [&mut time_zone],
        )?;
        let mut time_zone = time_zone.unwrap();
        if time_zone.len() == 0 {
            time_zone = "America/Toronto".to_owned();
        }

        let mut username_input = StringInput::new("Username", 24);
        InputWindow::run(
            console,
            "Set Username",
            "Enter the name for the initial user. Defaults to \"lhart\"",
            &mut [&mut username_input],
        )?;
        let mut username = username_input.unwrap().trim().to_owned();
        if username.len() == 0 {
            username = "lhart".to_owned();
        }

        Ok(CHRoot {
            time_zone,
            username,
        })
    }

    fn confirm(&self) -> Vec<(&str, String)> {
        vec![
            ("Time Zone", self.time_zone.clone()),
            ("Username", self.username.clone()),
        ]
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
            .args([self.time_zone, self.username])
            .run()
            .map_err(|error| CHRootError::CHRoot(error))
    }
}

impl Error for CHRootConfigurationError {
    fn is_curses_error(&self) -> bool {
        true
    }

    fn into_curses_error(self) -> Option<CursesError> {
        Some(self.0)
    }
}

impl Display for CHRootConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<CursesError> for CHRootConfigurationError {
    fn from(error: CursesError) -> Self {
        CHRootConfigurationError(error)
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
