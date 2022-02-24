#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("This panic was generated from a children thread.");
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
