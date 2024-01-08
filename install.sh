#!/bin/bash

# Installs ARCH linux on the system using an interactive guide

# Verify the system is in a good state to install
./verification/boot-mode.sh
./verification/internet.sh

# Get the user selected options

# Gets the user input
#   USAGE: user-input PROMPT DEFAULT 
#   Writes to $USER_INPUT
function user-input() {
    echo -n $1 "(Default: $2): "
    read USER_INPUT
    if [ "$USER_INPUT" = "" ]
    then
        USER_INPUT=$2
    fi
}

user-input "Enter the drive you wish to use" "/dev/sda"
DRIVE=$USER_INPUT

MEMORY_KB=$(grep MemTotal /proc/meminfo | awk '{print $2}')
MEMORY_GB=$((($MEMORY_KB + 1048575) / 1048576))
user-input "Enter the swap size in gigabytes" $MEMORY_GB
SWAP_SIZE=$USER_INPUT

LOCAL_TIME_PATH=$(readlink -f /etc/localtime)
LOCAL_TIME=${LOCAL_TIME_PATH#"/usr/share/zoneinfo/"}
user-input "Enter your timezone as \"Region/City\"" $LOCAL_TIME
TIME_ZONE=$USER_INPUT

user-input "Enter the hostname" "lss-lab-unnamed"
HOSTNAME=$USER_INPUT

# Confirm the users input
echo "The following settings will be used for this install:"
echo "    Drive: $DRIVE"
echo "    Swap Size: $SWAP_SIZE GB"
echo "    Timezone: $TIME_ZONE"
echo "    Hostname: $HOSTNAME"
echo -n "Do you wish to proceed? [Y/n] "
read CONFIRM
if [ ! $CONFIRM = ^[Yy]$ ]
then
    echo "Aborting . . ."
    exit -1
fi