#!/bin/bash

BOOT_PARTITION="$1"1
SWAP_PARTITION="$1"2
ROOT_PARTITION="$1"3

SWAP_SIZE=+"$2"G

echo "Partitioning $1 . . ."

sed -e 's/\s*\([\+0-9a-zA-Z]*\).*/\1/' << EOF | fdisk ${1} > /dev/null
    g # Create a new GUID partition table
    n # Create the boot partition
     # Default partition number 
     # Default start sector
    +1G # Set the size to 1GB

    n # Create the swap partition
     # Default partition number 
     # Default start sector
    $SWAP_SIZE # Set the size to the swap size

    n # Create the root partition
     # Default partition number 
     # Default start sector
     # Default size (remainder of the disk)

    w # Write the partitions to disk
    q # Quit
EOF