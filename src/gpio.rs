// gpio.rs
pub struct GpioPin {
    port: *mut u8, // pointer to the PORT register
    ddr: *mut u8,  // pointer to the DDR register
    pin: u8,       // pin number
}

pub enum PinMode {
    Input,
    Output,
}

impl GpioPin {
    /// initialize a new GPIO pin with the specified mode
    pub fn new(port: *mut u8, ddr: *mut u8, pin: u8, mode: PinMode) -> Self {
        let gpio_pin = GpioPin { port, ddr, pin };
        gpio_pin.set_mode(mode);
        gpio_pin
    }

    /// configure the pin mode (input or output)
    pub fn set_mode(&self, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => {
                    *self.ddr &= !(1 << self.pin); // set the pin as input
                }
                PinMode::Output => {
                    *self.ddr |= 1 << self.pin; // set the pin as output
                }
            }
        }
    }

    /// write a logic level (high/low) to the pin
    pub fn write(&self, high: bool) {
        unsafe {
            if high {
                *self.port |= 1 << self.pin; // high state
            } else {
                *self.port &= !(1 << self.pin); // low state
            }
        }
    }

    /// read the logic state of the pin
    pub fn read(&self) -> bool {
        unsafe { (*self.port & (1 << self.pin)) != 0 }
    }
}

/// a structure dedicated to configuring SPI pins
pub struct SpiPins {
    pub mosi: GpioPin,
    pub miso: GpioPin,
    pub sck: GpioPin,
    pub ss: GpioPin,
}

impl SpiPins {
    /// initialize the SPI pins with their respective roles
    pub fn new(port: *mut u8, ddr: *mut u8) -> Self {
        SpiPins {
            mosi: GpioPin::new(port, ddr, 3, PinMode::Output), // MOSI (PB3) as output
            miso: GpioPin::new(port, ddr, 4, PinMode::Input),  // MISO (PB4) as input
            sck: GpioPin::new(port, ddr, 5, PinMode::Output),  // SCK (PB5) as output
            ss: GpioPin::new(port, ddr, 2, PinMode::Output),   // SS (PB2) as output
        }
    }

    /// configure the SS pin to activate or deactivate the slave
    pub fn set_ss(&self, active: bool) {
        self.ss.write(!active); // active = LOW (0), inactive = HIGH (1)
    }

    /// reset the SPI pins to a safe state
    pub fn reset(&self) {
        self.mosi.write(false);
        self.sck.write(false);
        self.set_ss(true); // deactivate SS by default
    }

}
