#!/bin/bash

set -e

echo "Setting the time zone to $1 . . ."
ZONE_FILE=/usr/share/zoneinfo/$1
ln -sf $ZONE_FILE /etc/localtime

echo "Synchronizing the clocks . . ."
hwclock --systohc