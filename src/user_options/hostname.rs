use crate::{print, println, Console};

/// Initial user details
#[derive(Clone)]
pub(crate) struct Hostname(String);

impl Hostname {
    /// Gets the hostname for the system
    pub(super) fn get(console: &mut Console) -> Self {
        loop {
            print!(console, "Enter the hostname: ");
            let hostname = console.readln();

            if hostname.len() == 0 {
                println!(console, "Error: hostname missing, a hostname is required");
                continue;
            }

            return Hostname(hostname);
        }
    }
}

impl std::fmt::Display for Hostname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
