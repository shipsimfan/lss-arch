#!/bin/bash

set -e

echo "Creating the user . . ."
useradd -m -G wheel $1