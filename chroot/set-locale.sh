#!/bin/bash

set -e

echo "Generating locales . . ."
cp /root/chroot/locale.gen /etc/locale.gen
locale-gen > /dev/null

echo "Setting the locale . . ."
cp /root/chroot/locale.conf /etc/locale.conf