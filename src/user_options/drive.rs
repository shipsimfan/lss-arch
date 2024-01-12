use super::SwapSize;
use crate::{prompt, Console};
use std::{
    borrow::Cow,
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// The drive to install to
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Drive(Cow<'static, Path>);

/// The default drive to install to when the user doesn't specify
const DEFAULT_DRIVE: &str = "/dev/sda";

impl Drive {
    /// Gets the drive from the user
    pub(super) fn get(console: &mut Console) -> Self {
        prompt!(console, DEFAULT_DRIVE, "Enter the drive to install to");

        let drive = console.readln();

        Drive(if drive.len() == 0 {
            Cow::Borrowed(Path::new(DEFAULT_DRIVE))
        } else {
            Cow::Owned(drive.into())
        })
    }

    pub(crate) fn boot_partition(&self) -> PathBuf {
        self.0.with_file_name(format!(
            "{}1",
            self.0.file_name().unwrap().to_string_lossy()
        ))
    }

    pub(crate) fn swap_partition(&self) -> PathBuf {
        self.0.with_file_name(format!(
            "{}2",
            self.0.file_name().unwrap().to_string_lossy()
        ))
    }

    pub(crate) fn root_partition(&self, swap_size: SwapSize) -> PathBuf {
        if swap_size.is_some() {
            self.0.with_file_name(format!(
                "{}3",
                self.0.file_name().unwrap().to_string_lossy()
            ))
        } else {
            self.swap_partition()
        }
    }
}

impl std::fmt::Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

impl AsRef<OsStr> for Drive {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}
