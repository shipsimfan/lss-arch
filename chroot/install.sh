#!/bin/bash

set -e

# Script usage:
# ./install.sh TIME_ZONE HOSTNAME USERNAME
TIME_ZONE=$1
HOSTNAME=$2
USERNAME=$3

# Setup basic system settings
/root/chroot/set-timezone.sh $TIME_ZONE
/root/chroot/set-locale.sh
/root/chroot/set-hostname.sh $HOSTNAME

# Create the user
/root/chroot/user.sh $USERNAME

# Setup system services
mkdir -p /etc/systemd/system-preset
/root/chroot/sudo.sh
/root/chroot/ssh.sh $USERNAME
/root/chroot/dhcpcd.sh

# Install the bootloader
/root/chroot/bootloader.sh

# Remove chroot scripts when complete
rm -rf /root/chroot