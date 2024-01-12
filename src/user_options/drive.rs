use crate::{prompt, Console};
use std::{borrow::Cow, path::Path};

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

        Drive(if drive.trim().len() == 0 {
            Cow::Borrowed(Path::new(DEFAULT_DRIVE))
        } else {
            Cow::Owned(drive.into())
        })
    }
}

impl std::fmt::Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}
