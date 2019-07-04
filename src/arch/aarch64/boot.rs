#![no_std]
// #![feature(global_asm)]

/// Initializes the bss section before calling `_start()`.
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    extern "Rust" {
        fn _start() -> !;
    }

   _start();
}

// Disable all cores except core 0, and then jump to reset()
global_asm!(include_str!("boot.S"));