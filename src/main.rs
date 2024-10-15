#![no_std]
#![no_main]

use blinky::gpio::{GpioPin, PinMode};
use avr_device::atmega328p::Peripherals;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let portb = &dp.PORTB;

    let mut pin = GpioPin::new(portb, 2, PinMode::Output); // Configuration de la broche 2 en sortie
    pin.write(true); // Activer la broche

    loop {}
}
