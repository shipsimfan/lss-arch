#!/bin/bash

set -e

TIME_ZONE=$1
USERNAME=$2

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

# Enable SSH
ln -sf /usr/lib/systemd/system/sshd.service /etc/systemd/system/multi-user.target.wants/sshd.service

# Create user
useradd -m -G wheel $USERNAME
usermod -p '*' $USERNAME

# Install SSH keys
mkdir /home/$USERNAME/.ssh
chmod 700 /home/$USERNAME/.ssh

cp /root/authorized_keys /home/$USERNAME/.ssh/authorized_keys
chmod 600 /home/$USERNAME/.ssh/authorized_keys

chown -R $USERNAME:$USERNAME /home/$USERNAME/.ssh

# Install bootloader
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=LSS-ARCH
grub-mkconfig -o /boot/grub/grub.cfg