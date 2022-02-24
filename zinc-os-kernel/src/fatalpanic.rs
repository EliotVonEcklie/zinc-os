use core::panic::PanicInfo;

pub fn panic(info: &PanicInfo) -> ! {
    use crate::println;
    use crate::vga_buffer::{ WRITER, Color };

    WRITER.lock().change_color(Color::White, Color::Red);
    WRITER.lock().clear_screen();
    
    println!(
        "The kernel has panicked because a problem has been detected.\nYour system has been shut down to avoid damage.\nPlease, analize the error information provided in this screen and then, restart your system.\n\n\tError Information: {}",
        info
    );

    loop {}
}
