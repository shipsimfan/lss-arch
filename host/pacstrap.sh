#!/bin/bash

set -e

packstrap -K /mnt $(cat $1 | grep -v '^#')