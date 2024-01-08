#!/bin/bash

set -e

echo "Installing the bootloader . . ."
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=LSS-ARCH > /dev/null

echo "Creating the bootloader configuration . . ."
grub-mkconfig -o /boot/grub/grub.cfg > /dev/null