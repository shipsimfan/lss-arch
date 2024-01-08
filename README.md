# LSS Arch
An installer for Arch Linux for LSS servers.

## Installing
To install Arch Linux:
 1. Download the [Arch Linux install ISO](https://archlinux.org/download/) and boot the target
    machine with it.
 2. Clone this repository using git
 3. Run `install.sh` and follow the instructions

`packages.list` contains the list of packages that are installed with Arch Linux. To add any 
packages, add a newline to the end of the file with the name of package.

Any files that match `*.pub` in the `chroot/keys` folder will be used as SSH keys for the created 
user.