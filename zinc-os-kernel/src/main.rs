#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(asm_const)]
#![test_runner(zinc_os_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod fatalpanic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    {
        use vga_buffer::{ WRITER, Color };
        WRITER.lock().change_color(Color::LightGrey, Color::Black);
    }

    println!("ZincOS Kernel v0.1.0");

    #[cfg(test)]
    test_main();

    for i in 0..2000 {
        println!("{:#?}", i)
    }

    panic!("Mayday! Mayday! Kernel Panic Test!")

    //loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fatalpanic::panic(info);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zinc_os_kernel::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
