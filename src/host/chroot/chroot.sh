#!/bin/bash

set -e

TIME_ZONE=$1

# Set timezone
ln -sf /usr/share/zoneinfo/$TIME_ZONE /etc/localtime
hwclock --systohc

# Generate locales
locale-gen

# Install bootloader
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=LSS-ARCH
grub-mkconfig -o /boot/grub/grub.cfg