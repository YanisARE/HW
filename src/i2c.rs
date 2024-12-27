use core::ptr;

#[cfg(target_arch = "avr")] // specific to the AVR architecture
pub struct I2c {
    twbr: *mut u8,   // TWI Bit Rate Register
    twcr: *mut u8,   // TWI Control Register
    twdr: *mut u8,   // TWI Data Register
    twsr: *mut u8,   // TWI Status Register
    twar: *mut u8,   // TWI Address Register
}

#[cfg(target_arch = "avr")]
impl I2c {
    pub fn new() -> Self {
        I2c {
            twbr: 0xB8 as *mut u8,  // address of the TWBR register
            twcr: 0xBC as *mut u8,  // address of the TWCR register
            twdr: 0xBB as *mut u8,  // address of the TWDR register
            twsr: 0xB9 as *mut u8,  // address of the TWSR register
            twar: 0xBA as *mut u8,  // address of the TWAR register
        }
    }

    /// initialize the I2C (TWI)
    pub fn init(&self) {
        unsafe {
            ptr::write_volatile(self.twbr, 0x48);  // baud rate: calculated based on F_CPU
            ptr::write_volatile(self.twcr, 0x04);  // enable I2C (TWI)
        }
    }

    /// send I2C data
    pub fn write(&self, address: u8, data: u8) {
        unsafe {
            // load the slave address with the write bit
            ptr::write_volatile(self.twar, address << 1);

            // transfer the data
            ptr::write_volatile(self.twdr, data);

            // start the transmission and wait for it to finish
            ptr::write_volatile(self.twcr, 0xA4);  // start the transmission (TWSTA = 1)
            while (ptr::read_volatile(self.twsr) & 0xF8) != 0x28 {}  // wait for 'Transmission OK' status
        }
    }

    /// read data via I2C
    pub fn read(&self, address: u8) -> u8 {
        unsafe {
            // load the slave address with the read bit
            ptr::write_volatile(self.twar, (address << 1) | 0x01);

            // start the reception and wait for the end of the transmission
            ptr::write_volatile(self.twcr, 0xA4);  // start the reception (TWSTA = 1)
            while (ptr::read_volatile(self.twsr) & 0xF8) != 0x40 {}  // wait for 'Reception OK' status

            // read and return the received data
            ptr::read_volatile(self.twdr)
        }
    }

    /// check the I2C status (useful for error handling)
    pub fn check_status(&self) -> u8 {
        unsafe {
            ptr::read_volatile(self.twsr)  // return the current status
        }
    }
}

#[cfg(target_arch = "arm")] // specific to the ARM architecture (STM32 or other Cortex-M)
pub struct I2c {
    i2c_base: *mut u32,  // base address for the I2C controller
}

#[cfg(target_arch = "arm")]
impl I2c {
    pub fn new() -> Self {
        I2c {
            i2c_base: 0x40005400 as *mut u32, // example: base address for I2C1 on STM32F4
        }
    }

    /// initialize the I2C (master mode)
    pub fn init(&self) {
        unsafe {
            let cr1 = self.i2c_base.offset(0x00 / 4); // CR1 register
            let cr2 = self.i2c_base.offset(0x04 / 4); // CR2 register

            // configure the registers to enable I2C
            *cr1 |= (1 << 0);  // enable I2C
            *cr2 |= (1 << 0);  // enable event interrupt (if necessary)
        }
    }

    /// send I2C data
    pub fn write(&self, address: u8, data: u8) {
        unsafe {
            let dr = self.i2c_base.offset(0x10 / 4); // data register
            let sr1 = self.i2c_base.offset(0x14 / 4); // status register 1

            // load the slave address and the data to send
            while (*sr1 & (1 << 1)) == 0 {} // wait for I2C to be ready for sending
            *dr = (address << 1) as u32;  // slave address with the write bit

            while (*sr1 & (1 << 7)) == 0 {} // wait for the register to be ready
            *dr = data as u32;  // send the data
        }
    }

    /// read I2C data
    pub fn read(&self, address: u8) -> u8 {
        unsafe {
            let dr = self.i2c_base.offset(0x10 / 4); // data register
            let sr1 = self.i2c_base.offset(0x14 / 4); // status register 1

            // load the slave address with the read bit
            while (*sr1 & (1 << 1)) == 0 {}  // wait for I2C to be ready
            *dr = ((address << 1) | 1) as u32;  // slave address with the read bit

            while (*sr1 & (1 << 7)) == 0 {}  // wait for the data to be ready
            *dr as u8  // return the read data
        }
    }

    /// check the I2C status (useful for error handling)
    pub fn check_status(&self) -> u32 {
        unsafe {
            let sr1 = self.i2c_base.offset(0x14 / 4); // status register 1
            *sr1  // return the current status
        }
    }
}
