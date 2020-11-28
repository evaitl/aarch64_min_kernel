#![no_std]
#![no_main]
use core::panic::PanicInfo;
const UART0:*mut u8= 0x0900_0000 as *mut u8;
static HELLO: &[u8] = b"Hello world from the kernel\n";
fn write_serial(ch: u8) {
    unsafe {
        ptr::write_volatile(UART0,ch);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    loop {
        for (_i, &byte) in HELLO.iter().enumerate() {
            write_serial(byte);
        }
    }
}
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
