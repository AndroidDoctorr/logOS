#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
mod mem_utils;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
// Entry point
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    loop {}
}