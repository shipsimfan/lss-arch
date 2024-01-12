use crate::{println, Console, Drive, SwapSize};
use fdisk::FDisk;

mod fdisk;

/// Partitions `drive` using fdisk
pub(crate) fn partition(
    console: &mut Console,
    drive: &Drive,
    swap_size: SwapSize,
) -> std::io::Result<()> {
    println!(console, "Partitioning {} . . .", drive);

    let mut fdisk = FDisk::spawn(drive)?;
    fdisk.create_guid_table()?;

    // Boot partition
    fdisk.create_partition(Some("+1G"), Some("1"))?;

    // Swap partition
    if let Some(swap_size) = *swap_size {
        fdisk.create_partition(Some(&format!("+{}G", swap_size)), Some("19"))?;
    }

    // Root partition
    fdisk.create_partition(None, None)?;

    fdisk.finalize()
}
