#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zinc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    {
        use vga_buffer::{ WRITER, Color };
        WRITER.lock().change_color(Color::LightGray, Color::Black);
    }

    println!("ZincOS Kernel v0.1.0");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zinc_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
