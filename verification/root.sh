#!/bin/bash

if [ ! "$EUID" -eq 0 ]
then
    echo "Currently not running as root. Please run this script as root"
    exit 1
fi