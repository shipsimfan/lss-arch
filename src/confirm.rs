use crate::{print, println, Console, Internet, UserOptions};

/// The user input was not confirmed
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct NotConfirmedError;

/// Gets the user to confirm the provided options
pub(crate) fn confirm(
    options: &UserOptions,
    console: &mut Console,
) -> Result<(), NotConfirmedError> {
    println!(
        console,
        "The following options will be used for this install:"
    );
    println!(console, "  Drive: {}", options.drive());
    println!(console, "  Swap Size: {}", options.swap_size());
    println!(console, "  TimeZone: {}", options.time_zone());
    println!(console, "  Username: {}", options.username());
    println!(console, "  Hostname: {}", options.hostname());
    match options.internet() {
        Internet::DHCP => println!(console, "  IP Address: DHCP"),
        Internet::Static {
            device,
            ip,
            gateway,
            nameservers,
        } => {
            println!(console, "  Network Interface: {}", device);
            println!(console, "  IP Address: {}", ip);
            println!(console, "  Gateway: {}", gateway);

            print!(console, "  Nameservers: ");
            if nameservers.len() == 0 {
                print!(console, "None")
            } else {
                let mut first = true;
                for nameserver in nameservers {
                    if first {
                        first = false;
                    } else {
                        print!(console, ", ");
                    }

                    print!(console, "{}", nameserver);
                }
            }
            println!(console);
        }
    }

    print!(console, "Do you wish to proceed? [Y/n] ");
    let confirm = console.readln();
    if confirm.len() > 0 && confirm.as_bytes()[0].to_ascii_lowercase() == b'y' {
        Ok(())
    } else {
        Err(NotConfirmedError)
    }
}

impl std::error::Error for NotConfirmedError {}

impl std::fmt::Display for NotConfirmedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "install options not confirmed, aborting")
    }
}

impl std::fmt::Debug for NotConfirmedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
