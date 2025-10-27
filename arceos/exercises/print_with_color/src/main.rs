#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::with_color;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    with_color!(31, "[WithColor]: Hello, Arceos!");
}
