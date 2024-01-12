use crate::{println, prompt, Console};
use std::num::NonZeroU8;

/// The size of the swap partition in gigabytes
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct SwapSize(Option<NonZeroU8>);

const DEFAULT_SWAP_SIZE: usize = 0;

impl SwapSize {
    /// Gets the size of the swap partition from the user
    pub(super) fn get(console: &mut Console) -> Self {
        let swap_size = loop {
            prompt!(
                console,
                DEFAULT_SWAP_SIZE,
                "Enter the swap size in gigabytes"
            );

            let swap_size = console.readln();

            if swap_size.trim().len() == 0 {
                break 0;
            }

            match swap_size.trim().parse() {
                Ok(swap_size) => break swap_size,
                Err(_) => {
                    println!(console, "Error: invalid size entered");
                }
            }
        };

        SwapSize(NonZeroU8::new(swap_size))
    }
}

impl std::fmt::Display for SwapSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(swap_size) => write!(f, "{} GB", swap_size),
            None => f.write_str("no swap partition"),
        }
    }
}
