#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

// in main.rs

#[cfg(test)]
mod qemu;
mod serial;
mod vga_buffer;

#[cfg(not(test))]
use core::fmt::Write;
#[cfg(test)]
use qemu::{exit_qemu, QemuExitCode};
#[cfg(not(test))]
use vga_buffer::{Color, ColorCode, Writer};

use core::panic::PanicInfo;

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(ColorCode::new(Color::Red, Color::Black));
    writer.write_fmt(format_args!("{}", info)).unwrap();
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_runner(tests: &[&dyn Fn()]) {
        serial_println!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
        exit_qemu(QemuExitCode::Success);
    }

    #[test_case]
    fn trivial_assertion() {
        serial_print!("trivial assertion... ");
        assert_eq!(1, 1);
        serial_println!("[ok]");
    }
}
