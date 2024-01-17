use std::ffi::CStr;

use crate::console::{Console, CursesError, IPv4Input, InputWindow, U8Input};
use net_utils::ip::v4::{IPv4Address, IPv4CIDR};

pub struct StaticIP {
    device: String,
    ip: IPv4CIDR,
    gateway: Option<IPv4Address>,
    primary_nameserver: Option<IPv4Address>,
    secondary_nameserver: Option<IPv4Address>,
}

const RESOLV_CONF_HEADER: &str =
    "# Resolver configuration file.\n# See resolv.conf(5) for details.\n\n";

const SET_IP_SH_PATH: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"/mnt/root/set-ip.sh\0") };

const SET_IP_SERVICE: &[u8] = include_bytes!("set-ip.service");

impl StaticIP {
    pub(super) fn configure(device: String, console: &mut Console) -> Result<Self, CursesError> {
        let mut ip_input = IPv4Input::new("IP");
        let mut subnet_input = U8Input::new("Subnet size");
        let mut gateway_input = IPv4Input::new("Gateway");
        let mut primary_nameserver_input = IPv4Input::new("Primary DNS");
        let mut secondary_nameserver_input = IPv4Input::new("Secondary DNS");

        InputWindow::run(
            console,
            "Static IP",
            "Enter the static IP, gateway, and nameservers",
            &mut [
                &mut ip_input,
                &mut subnet_input,
                &mut gateway_input,
                &mut primary_nameserver_input,
                &mut secondary_nameserver_input,
            ],
        )?;

        let ip = IPv4CIDR::new(
            ip_input.unwrap(),
            subnet_input.unwrap().unwrap_or(0).min(32),
        );

        let gateway = gateway_input.unwrap();
        let gateway = if gateway == IPv4Address::UNSPECIFIED {
            None
        } else {
            Some(gateway)
        };

        let primary_nameserver = primary_nameserver_input.unwrap();
        let primary_nameserver = if primary_nameserver == IPv4Address::UNSPECIFIED {
            None
        } else {
            Some(primary_nameserver)
        };

        let secondary_nameserver = secondary_nameserver_input.unwrap();
        let secondary_nameserver = if secondary_nameserver == IPv4Address::UNSPECIFIED {
            None
        } else {
            Some(secondary_nameserver)
        };

        Ok(StaticIP {
            device,
            ip,
            gateway,
            primary_nameserver,
            secondary_nameserver,
        })
    }

    pub(super) fn confirm(&self) -> Vec<(&str, String)> {
        let mut confirm = Vec::with_capacity(5);
        confirm.push(("Interface", self.device.clone()));
        confirm.push(("IP", self.ip.to_string()));

        if let Some(gateway) = self.gateway {
            confirm.push(("Gateway", gateway.to_string()));
        }

        if let Some(primary_nameserver) = self.primary_nameserver {
            confirm.push(("Primary DNS", primary_nameserver.to_string()));
        }

        if let Some(secondary_nameserver) = self.secondary_nameserver {
            confirm.push(("Secondary DNS", secondary_nameserver.to_string()));
        }

        confirm
    }

    pub(super) fn install(self) -> std::io::Result<()> {
        // Write "/root/set-ip.sh"
        let mut set_ip_sh = format!(
            "#!/bin/bash\n\nset -e\n\nip link set up dev {}\nip address add {} broadcast + dev {}\n",
            self.device, self.ip,  self.device
        );
        if let Some(gateway) = self.gateway {
            set_ip_sh.push_str(&format!(
                "ip route add 0.0.0.0/0 via {} dev {}\n",
                gateway, self.device
            ));
        }
        std::fs::write("/mnt/root/set-ip.sh", set_ip_sh)?;
        unsafe { linux::sys::stat::chmod(SET_IP_SH_PATH.as_ptr(), 0o744) };

        // Write "/etc/systemd/system/set-ip.service"
        std::fs::write("/mnt/etc/systemd/system/set-ip.service", SET_IP_SERVICE)?;

        // Write "/etc/resolv.conf"
        if self.primary_nameserver.is_some() || self.secondary_nameserver.is_some() {
            let mut resolv_conf = RESOLV_CONF_HEADER.to_owned();

            if let Some(primary_nameserver) = self.primary_nameserver {
                resolv_conf.push_str(&format!("nameserver {}\n", primary_nameserver));
            }

            if let Some(secondary_nameserver) = self.secondary_nameserver {
                resolv_conf.push_str(&format!("nameserver {}\n", secondary_nameserver));
            }

            std::fs::write("/mnt/etc/resolv.conf", resolv_conf)?;
        }

        Ok(())
    }
}
