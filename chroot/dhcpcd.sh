#!/bin/bash

set -e

echo "Enabling DHCPCD service . . ."
cp /root/chroot/10-dhcpcd.preset /etc/systemd/system-preset/10-dhcpcd.preset