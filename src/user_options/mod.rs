use crate::Console;
use drive::Drive;
use hostname::Hostname;
use swap_size::SwapSize;
use timezone::TimeZone;
use username::Username;

mod drive;
mod hostname;
mod internet;
mod swap_size;
mod timezone;
mod username;

pub(crate) use internet::Internet;

/// The installation options the user selected
pub(crate) struct UserOptions {
    drive: Drive,
    swap_size: SwapSize,
    time_zone: TimeZone,
    username: Username,
    hostname: Hostname,
    internet: Internet,
}

impl UserOptions {
    /// Gets the options from the user
    pub(crate) fn get(console: &mut Console) -> Self {
        let drive = Drive::get(console);
        let swap_size = SwapSize::get(console);
        let time_zone = TimeZone::get(console);
        let username = Username::get(console);
        let hostname = Hostname::get(console);
        let internet = Internet::get(console);

        UserOptions {
            drive,
            swap_size,
            username,
            time_zone,
            hostname,
            internet,
        }
    }

    /// Gets the drive to install to
    pub(crate) fn drive(&self) -> &Drive {
        &self.drive
    }

    /// Gets the size of the swap partition
    pub(crate) fn swap_size(&self) -> SwapSize {
        self.swap_size
    }

    /// Gets the initial user's username
    pub(crate) fn username(&self) -> &Username {
        &self.username
    }

    /// Gets the time zone for the new system
    pub(crate) fn time_zone(&self) -> &TimeZone {
        &self.time_zone
    }

    /// Gets the hostname for the new system
    pub(crate) fn hostname(&self) -> &Hostname {
        &self.hostname
    }

    /// Gets the internet settings for the new system
    pub(crate) fn internet(&self) -> &Internet {
        &self.internet
    }
}
