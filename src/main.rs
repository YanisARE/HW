#![no_std]
#![no_main]

mod gpio;
use gpio::{GpioPin, PinMode};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    let mut output_pin = GpioPin::new(PORTB, DDRB, 2, PinMode::Output);
    let input_pin_pullup = GpioPin::new(PORTB, DDRB, 3, PinMode::InputPullUp);

    output_pin.write(true);

    loop {
        let input_state = input_pin_pullup.read();
        if input_state {
            output_pin.write(false);
        } else {
            output_pin.write(true);
        }
    }
}
