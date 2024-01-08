#!/bin/bash

BOOT_PARTITION="$1"1
SWAP_PARTITION="$1"2
ROOT_PARTITION="$1"3

echo "Formatting $BOOT_PARTITION as FAT 32 . . ."
mkfs.vfat -F 32 $BOOT_PARTITION > /dev/null

echo "Formatting $SWAP_PARTITION as swap . . ."
mkswap $SWAP_PARTITION -q

echo "Formatting $ROOT_PARTITION as ext4 . . ."
mkfs.ext4 $ROOT_PARTITION -q -F