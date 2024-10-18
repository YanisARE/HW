#![no_std]
#![no_main]

use cortex_m_rt::entry;

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const MESSAGE: *mut u8 = 0x100 as *mut u8;

#[entry]
fn main() -> ! {
    unsafe {
        core::ptr::write_volatile(DDRB, 0b00010000);

        loop {
            let on_msg = b"allum\0";
            for (i, &byte) in on_msg.iter().enumerate() {
                core::ptr::write_volatile(MESSAGE.offset(i as isize), byte);
            }

            for _ in 0..1_000_000 {
                core::ptr::write_volatile(PORTB, 0b00000000);
            }

            let off_msg = b"eteint\0";
            for (i, &byte) in off_msg.iter().enumerate() {
                core::ptr::write_volatile(MESSAGE.offset(i as isize), byte);
            }

            for _ in 0..100_000 {
                core::ptr::write_volatile(PORTB, 0b11111111);
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
