#![no_std]
#![no_main]

use panic_halt as _; // Panic handler
use avr_device::atmega328p::Peripherals;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let portb = &dp.PORTB;

    // Configurer PB4 comme sortie
    portb.ddrb.write(|w| w.pb4().set_bit());

    loop {
        // Allumer PB4
        portb.portb.write(|w| w.pb4().set_bit());
        for _ in 0..1_000_000 {}

        // Éteindre PB4
        portb.portb.write(|w| w.pb4().clear_bit());
        for _ in 0..1_000_000 {}
    }
}
