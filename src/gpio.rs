// src/gpio.rs

use avr_device::atmega328p::PORTB; // import du port B pour l'exemple

pub enum PinMode {
    Input,
    Output,
    InputPullUp,
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
                unsafe {
                    self.port.ddrb.modify(|r, w| w.bits(r.bits() & !(1 << self.pin)));
                    self.port.portb.modify(|r, w| w.bits(r.bits() & !(1 << self.pin)));
                }
            }
            PinMode::Output => {
                unsafe {
                    self.port.ddrb.modify(|r, w| w.bits(r.bits() | (1 << self.pin)));
                }
            }

            PinMode::InputPullUp => {
                unsafe {
                    self.port.ddrb.modify(|r, w| w.bits(r.bits() & !(1 << self.pin))); // Entrée
                    self.port.portb.modify(|r, w| w.bits(r.bits() | (1 << self.pin))); // Pull-up activé
                }
            }
        }
    }

    pub fn write(&self, high: bool) {
        unsafe {
            if high {
                self.port.portb.modify(|r, w| w.bits(r.bits() | (1 << self.pin)));
            } else {
                self.port.portb.modify(|r, w| w.bits(r.bits() & !(1 << self.pin)));
            }
        }
    }

    pub fn read(&self) -> bool {
        let pin_value = unsafe { self.port.pinb.read().bits() };
        (pin_value & (1 << self.pin)) != 0
    }
}
