#!/bin/bash

set -e

echo "Enabling DHCPCD service . . ."
cp /root/chroot/10-dhcpcd.preset /etc/systemd/system-preset/10-dhcpcd.preset
ln -sf /usr/lib/systemd/system/dhcpcd.service /etc/systemd/multi-user.target.wants/dhcpcd.service
