use crate::{println, Console, Drive, SwapSize};
use std::process::Command;

const ROOT_MOUNT_DIR: &str = "/mnt";
const BOOT_MOUNT_DIR: &str = "/mnt/boot";

/// Mounts the partitions on `drive`
pub(crate) fn mount(
    console: &mut Console,
    drive: &Drive,
    swap_size: SwapSize,
) -> std::io::Result<()> {
    let root_partition = drive.root_partition(swap_size);
    println!(console, "Mounting {} . . .", root_partition.display());
    if !Command::new("mount")
        .arg(root_partition)
        .arg(ROOT_MOUNT_DIR)
        .status()?
        .success()
    {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    let boot_partition = drive.boot_partition();
    println!(console, "Mounting {} . . .", boot_partition.display());
    if !Command::new("mount")
        .arg(boot_partition)
        .arg(BOOT_MOUNT_DIR)
        .arg("--mkdir")
        .status()?
        .success()
    {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    if swap_size.is_some() {
        let swap_partition = drive.swap_partition();
        println!(console, "Mounting {} . . .", swap_partition.display());
        if !Command::new("swapon")
            .arg(swap_partition)
            .status()?
            .success()
        {
            return Err(std::io::Error::from(std::io::ErrorKind::Other));
        }
    }

    Ok(())
}
