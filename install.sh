#!/bin/bash

# Installs ARCH linux on the system using an interactive guide

set -e

# Verify the system is in a good state to install
./verification/root.sh
./verification/boot-mode.sh
./verification/internet.sh

# Get the user selected options

# Gets the user input
#   USAGE: user-input PROMPT DEFAULT 
#   Writes to $REPLY
function user-input() {
    echo -n 
    read -p "$1 (Default: $2): "
    if [ "$REPLY" = "" ]
    then
        REPLY=$2
    fi
}

user-input "Enter the drive you wish to use" "/dev/sda"
DRIVE=$REPLY

MEMORY_KB=$(grep MemTotal /proc/meminfo | awk '{print $2}')
MEMORY_GB=$((($MEMORY_KB + 1048575) / 1048576))
user-input "Enter the swap size in gigabytes" $MEMORY_GB
SWAP_SIZE=$REPLY

LOCAL_TIME_PATH=$(readlink -f /etc/localtime)
LOCAL_TIME=${LOCAL_TIME_PATH#"/usr/share/zoneinfo/"}
user-input "Enter your timezone as \"Region/City\"" $LOCAL_TIME
TIME_ZONE=$REPLY

user-input "Enter the hostname" "lss-lab-unnamed"
HOSTNAME=$REPLY

# Confirm the users input
echo
echo "The following settings will be used for this install:"
echo "    Drive: $DRIVE"
echo "    Swap Size: $SWAP_SIZE GB"
echo "    Timezone: $TIME_ZONE"
echo "    Hostname: $HOSTNAME"

read -p "Do you wish to proceed? [Y/n] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    echo "Aborting . . ."
    exit -1
fi

echo
echo "Installing Arch Linux to $DRIVE . . ."

# Parition, format, and mount the destination drive
./host/partition.sh $DRIVE $SWAP_SIZE
./host/format.sh $DRIVE
./host/mount.sh $DRIVE

# Install the required packages
./host/pacstrap.sh packages.list