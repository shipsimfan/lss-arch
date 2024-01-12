use crate::{print, Console};
use net_utils::ip::v4::{IPv4Address, IPv4CIDR};

/// Settings for the new systems internet connection
pub(crate) enum Internet {
    DHCP,
    Static {
        device: String,
        ip: IPv4CIDR,
        gateway: IPv4Address,
        nameservers: Vec<IPv4Address>,
    },
}

impl Internet {
    /// Gets the settings for the new systems internet connection
    pub(super) fn get(console: &mut Console) -> Self {
        print!(
            console,
            "Enter the device for internet connections (eg. eth0), or blank to use DHCP: "
        );
        let device = console.readln();

        if device.len() == 0 {
            return Internet::DHCP;
        }

        let ip = loop {
            print!(
                console,
                "Enter the IP address and subnet (eg. 10.0.0.2/24): "
            );
            let ip = console.readln();

            if ip.len() == 0 {
                println!("Error: missing IP address, IP address and subnet are required");
                continue;
            }

            match ip.parse() {
                Ok(ip) => break ip,
                Err(_) => println!("Error: invalid CIDR entered"),
            }
        };

        let gateway = loop {
            print!(console, "Enter the gateway: ");
            let gateway = console.readln();

            if gateway.len() == 0 {
                println!("Error: missing gateway, a gateway is required");
                continue;
            }

            match gateway.parse() {
                Ok(gateway) => break gateway,
                Err(_) => println!("Error: invalid IP entered"),
            }
        };

        let mut nameservers = Vec::with_capacity(2);

        loop {
            print!(console, "Enter the primary nameserver, or blank for none: ");
            let primary_nameserver = console.readln();

            if primary_nameserver.len() == 0 {
                break;
            }

            match primary_nameserver.parse() {
                Ok(nameserver) => {
                    nameservers.push(nameserver);
                    break;
                }
                Err(_) => println!("Error: invalid IP entered"),
            }
        }

        if nameservers.len() == 1 {
            loop {
                print!(
                    console,
                    "Enter the secondary nameserver, or blank for none: "
                );
                let secondary_nameserver = console.readln();

                if secondary_nameserver.len() == 0 {
                    break;
                }

                match secondary_nameserver.parse() {
                    Ok(nameserver) => {
                        nameservers.push(nameserver);
                        break;
                    }
                    Err(_) => println!("Error: invalid IP entered"),
                }
            }
        }

        Internet::Static {
            device,
            ip,
            gateway,
            nameservers,
        }
    }
}
