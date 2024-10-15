// src/gpio.rs

use avr_device::atmega328p::PORTB; // import du port B pour l'exemple

pub enum PinMode {
    Input,
    Output,
}

pub struct GpioPin<'a> {
    port: &'a PORTB,
    pin: u8,
}

impl<'a> GpioPin<'a> {
    pub fn new(port: &'a PORTB, pin: u8, mode: PinMode) -> Self {
        let gpio_pin = GpioPin { port, pin };
        gpio_pin.set_mode(mode);
        gpio_pin
    }

    pub fn set_mode(&self, mode: PinMode) {
        match mode {
            PinMode::Input => {
                // Configuration pour définir la broche en entrée
                // Exemple d'utilisation d'un registre
                unsafe { self.port.ddrb.write(|w| w.bits(0x00)); }
            }
            PinMode::Output => {
                // Configuration pour définir la broche en sortie
                unsafe { self.port.ddrb.write(|w| w.bits(0xFF)); }
            }
        }
    }

    pub fn write(&self, high: bool) {
        if high {
            unsafe { self.port.portb.write(|w| w.bits(1 << self.pin)); }
        } else {
            unsafe { self.port.portb.write(|w| w.bits(0)); }
        }
    }

    pub fn read(&self) -> bool {
        let pin_value = unsafe { self.port.pinb.read().bits() };
        (pin_value & (1 << self.pin)) != 0
    }
}
