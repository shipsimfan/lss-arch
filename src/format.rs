use crate::{println, Console, Drive, SwapSize};
use std::process::{Command, Stdio};

/// Formats the partitions on `drive`
pub(crate) fn format(
    console: &mut Console,
    drive: &Drive,
    swap_size: SwapSize,
) -> std::io::Result<()> {
    let boot_partition = drive.boot_partition();
    println!(
        console,
        "Formatting {} as FAT 32 . . .",
        boot_partition.display()
    );
    if !Command::new("mkfs.vfat")
        .args(&["-F", "32"])
        .arg(boot_partition)
        .stdout(Stdio::null())
        .status()?
        .success()
    {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    if swap_size.is_some() {
        let swap_partition = drive.swap_partition();
        println!(
            console,
            "Formatting {} as swap . . .",
            swap_partition.display()
        );
        if !Command::new("mkswap")
            .arg("-q")
            .arg(swap_partition)
            .stdout(Stdio::null())
            .status()?
            .success()
        {
            return Err(std::io::Error::from(std::io::ErrorKind::Other));
        }
    }

    let root_partition = drive.root_partition(swap_size);
    println!(
        console,
        "Formatting {} as ext4 . . .",
        root_partition.display()
    );
    if !Command::new("mkfs.ext4")
        .arg("-q")
        .arg(root_partition)
        .stdout(Stdio::null())
        .status()?
        .success()
    {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    Ok(())
}
