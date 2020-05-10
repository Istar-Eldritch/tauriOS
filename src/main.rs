
#![no_std]

#![no_main]

#![feature(asm)]
// in main.rs

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Writer, ColorCode, Color};
use core::fmt::Write;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(ColorCode::new(Color::Red, Color::Black));
    writer.write_fmt(format_args!("{}", info)).unwrap();
    loop {}
}
