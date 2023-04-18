#![no_std] // NOTE: Don't link the Rust standard library.
#![no_main] // NOTE: Disable all Rust-level entry points.

use core::panic::PanicInfo;

#[no_mangle] // NOTE: Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
  // NOTE: This function is the entry point, since the linker looks for a function
  //    named _start by default.
  loop {}
}

/// This function called to handle a panic condition.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
