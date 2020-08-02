#![no_main]
#![no_std]

#![feature(global_asm)]

use core::fmt::Write;

#[no_mangle]
pub static mut STACK0: [u8; 4096 * 3] = [0; 4096 * 3];

#[no_mangle]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut uart0 = uart::Uart::new(arch::riscv_virt::UART0);
    uart0.init();
    match uart0.write_str("Hello World from Rust(riscv)!\n") {
        Ok(()) => {},
        Err(_err) => {},
    };

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}

pub mod assembly;
pub mod arch;
pub mod uart;
