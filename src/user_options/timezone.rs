use crate::{prompt, Console};
use std::{borrow::Cow, path::Path};

/// The time zone for the new system
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct TimeZone(Cow<'static, Path>);

/// The default time zone for the new system
const DEFAULT_TIME_ZONE: &str = "America/Toronto";

impl TimeZone {
    /// Gets the time zone from the user
    pub(super) fn get(console: &mut Console) -> Self {
        prompt!(console, DEFAULT_TIME_ZONE, "Enter the your time zone");

        let time_zone = console.readln();

        TimeZone(if time_zone.len() == 0 {
            Cow::Borrowed(Path::new(DEFAULT_TIME_ZONE))
        } else {
            Cow::Owned(time_zone.into())
        })
    }
}

impl std::fmt::Display for TimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}
