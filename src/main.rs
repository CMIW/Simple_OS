#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

// This function is the new entry point, since we don't have an underlying runtime to call main
// and the linker looks for a function named `_start` by default.
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello World!").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}
