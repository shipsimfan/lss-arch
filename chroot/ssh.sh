#!/bin/bash

set -e

echo "Setting up SSH config . . ."
cp /root/chroot/sshd_config /etc/ssh/sshd_config

echo "Installing keys . . ."
mkdir /home/$1/.ssh
for key in /root/chroot/keys/*.pub; do
    [ -f "$i" ] || break

    cat $key >> /home/$1/.ssh/authorized_keys
done