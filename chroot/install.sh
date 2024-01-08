#!/bin/bash

set -e

# Script usage:
# ./install.sh TIMEZONE HOSTNAME USERNAME
TIME_ZONE=$1
HOSTNAME=$2
USERNAME=$3

echo "Time zone: $TIME_ZONE"
echo "Hostname: $HOSTNAME"
echo "Username: $USERNAME"