#!/bin/bash

set -e
shopt -s nullglob

echo "Setting up SSH config . . ."
cp /root/chroot/sshd_config /etc/ssh/sshd_config

echo "Enabling SSH service . . ."
cp /root/chroot/10-ssh.preset /etc/systemd/system-preset/10-ssh.preset

echo "Installing keys . . ."
mkdir -p /home/$1/.ssh
cd /root/chroot/keys; for key in *.pub; do
    cat $key >> /home/$1/.ssh/authorized_keys
done