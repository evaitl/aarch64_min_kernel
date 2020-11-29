#![no_std]
#![no_main]
#![feature(global_asm)]
use core::panic::PanicInfo;
const UART0:*mut u8= 0x0900_0000 as *mut u8;
static HELLO: &[u8] = b"Hello world from the kernel\n";
fn write_serial(ch: u8) {
    use core::ptr;
    unsafe {
        ptr::write_volatile(UART0,ch);
    }
}

global_asm!(r#"
.extern LD_STACK_PTR
.global _start
_start:
    ldr x30, =LD_STACK_PTR
    mov sp, x30
    b start
"#);

#[no_mangle]
pub unsafe extern "C" fn start() -> ! {
    loop {
        for byte in HELLO {
            write_serial(*byte);
        }
    }
}
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
