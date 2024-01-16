#!/bin/bash

set -e

# Install bootloader
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=LSS-ARCH
grub-mkconfig -o /boot/grub/grub.cfg