#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

fn wait(){
    for _ in 0..10_000_000{
        core::hint::spin_loop();
    }
}
fn printz(s: &str, color: u8){
    unsafe {
        vga_buffer::write_line(s, color);
    }
}


/// Esta función se llama cuando ocurre un pánico.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    printz("Hola bro", 0x0F);
    loop {}
}

