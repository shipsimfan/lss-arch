#!/bin/bash

set -e

# Script usage:
# ./install.sh TIME_ZONE HOSTNAME USERNAME
TIME_ZONE=$1
HOSTNAME=$2
USERNAME=$3

/root/chroot/set-timezone.sh $TIME_ZONE
/root/chroot/set-locale.sh