#![no_std]
use core::ptr;

pub enum PinMode {
    Output,
    InputPullUp,
}

pub struct GpioPin {
    port: *mut u8,
    ddr: *mut u8,
    pin: u8,
}

impl GpioPin {
    pub fn new(port: *mut u8, ddr: *mut u8, pin: u8, mode: PinMode) -> Self {
        let gpio = GpioPin { port, ddr, pin };
        unsafe {
            match mode {
                PinMode::Output => {
                    ptr::write_volatile(gpio.ddr, ptr::read_volatile(gpio.ddr) | (1 << gpio.pin));
                }
                PinMode::InputPullUp => {
                    ptr::write_volatile(gpio.ddr, ptr::read_volatile(gpio.ddr) & !(1 << gpio.pin)); // Configurer comme entrée
                    ptr::write_volatile(gpio.port, ptr::read_volatile(gpio.port) | (1 << gpio.pin)); // Activer la résistance pull-up
                }
            }
        }
        gpio
    }

    pub fn write(&self, state: bool) {
        unsafe {
            if state {
                ptr::write_volatile(self.port, ptr::read_volatile(self.port) | (1 << self.pin));
            } else {
                ptr::write_volatile(self.port, ptr::read_volatile(self.port) & !(1 << self.pin));
            }
        }
    }

    pub fn read(&self) -> bool {
        unsafe {
            let value = ptr::read_volatile(self.port);
            (value & (1 << self.pin)) != 0
        }
    }
}