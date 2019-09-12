#![feature(asm, const_fn, lang_items, global_asm)]
#![no_std]

//extern crate spin;
#[cfg(target_arch = "x86_64")]
extern crate x86;

#[cfg(target_arch = "aarch64")]
extern crate aarch64;

// These need to be visible to the linker, so we need to export them.
pub use logging::*;
#[cfg(target_arch = "x86_64")]
pub use arch::processor::*;

#[macro_use]
pub mod macros;
#[macro_use]
pub mod logging;
pub mod arch;
pub mod console;
