#!/bin/bash

set -e

echo "Setting hostname to $1 . . ."
echo $1 > /etc/hostname