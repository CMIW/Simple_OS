#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// This function is the new entry point, since we don't have an underlying runtime to call main
// and the linker looks for a function named `_start` by default.
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // The VGA buffer is located at the address 0xb8000
    // Cast the interger 0xb8000 into a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}
