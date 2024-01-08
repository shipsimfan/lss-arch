#!/bin/bash

set -e

# Make sure the keyring is updated
pacman -Sy archlinux-keyring --noconfirm

# Install the packages
pacstrap -K /mnt $(cat $1 | grep -v '^#')