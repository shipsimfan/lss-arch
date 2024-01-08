#!/bin/bash

set -e

echo "Verifying boot mode . . ."
if [ ! $(cat /sys/firmware/efi/fw_platform_size) -eq 64 ] ; then
    echo "Error: the system was not booted in 64-bit mode with UEFI"
    exit -1
fi