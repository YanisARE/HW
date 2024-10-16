#![no_std]
#![no_main]

use blinky::gpio::{GpioPin, PinMode};
use avr_device::atmega328p::Peripherals;
//use embedded_hal::digital::v2::OutputPin;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let portb = &dp.PORTB;//On prends le prot B comme exemple
    let mut output_pin = GpioPin::new(portb, 2, PinMode::Output);//On configure la broche 2 en sortie
    let input_pin_pullup = GpioPin::new(portb, 3, PinMode::InputPullUp);//ici on configure la broche 3 comme entrée avec resistance
    output_pin.write(true);//On active la broche 2
    //on simule l'état de broche 3 avec résistance pull-up activée
    loop{
        let input_state = input_pin_pullup.read();
        if input_state {
            output_pin.write(false);
        } else {
            output_pin.write(true);
        }
    }
}
