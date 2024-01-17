#!/bin/bash

set -e

TIME_ZONE=$1

# Set timezone
ln -sf /usr/share/zoneinfo/$TIME_ZONE /etc/localtime
hwclock --systohc

# Generate locales
locale-gen

# Setup internet
if test -f /root/set-ip.sh; then
    # Enable set-ip.service
    ln -sf /etc/systemd/system/set-ip.service /etc/systemd/system/multi-user.target.wants/set-ip.service
else
    # Enable dhcpcd
    ln -sf /usr/lib/systemd/system/dhcpcd.service /etc/systemd/system/multi-user.target.wants/dhcpcd.service
fi

# Install bootloader
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=LSS-ARCH
grub-mkconfig -o /boot/grub/grub.cfg