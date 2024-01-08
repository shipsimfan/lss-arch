#!/bin/bash

set -e

echo "Verifying internet connection . . ."
ping -c 1 archlinux.org &> /dev/null
if [ ! $? -eq 0 ] ; then
    echo "Error: cannot ping archlinux.org, verify you are connected to the internet and try again"
    exit -1
fi