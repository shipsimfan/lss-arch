#!/bin/bash

set -e

BOOT_PARTITION="$1"1
SWAP_PARTITION="$1"2
ROOT_PARTITION="$1"3

echo "Unmounting /mnt . . ."
swapoff -a
if cat /proc/mounts | grep /mnt
then
    umount -Rf /mnt
fi

echo "Mounting $ROOT_PARTITION . . ."
mount $ROOT_PARTITION /mnt

echo "Mounting $BOOT_PARTITION . . ."
mount $BOOT_PARTITION /mnt/boot --mkdir

echo "Mounting $SWAP_PARTITION . . ."
swapon $SWAP_PARTITION