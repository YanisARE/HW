use core::ptr;

pub struct Spi {
    spcr: *mut u8,   // SPI Control Register
    spsr: *mut u8,   // SPI Status Register
    spdr: *mut u8,   // SPI Data Register
}

impl Spi {
    pub fn new() -> Self {
        Spi {
            spcr: 0x4C as *mut u8, // Adresse du registre SPCR
            spsr: 0x4D as *mut u8, // Adresse du registre SPSR
            spdr: 0x4E as *mut u8, // Adresse du registre SPDR
        }
    }

    /// Initialiser SPI en mode Master
    pub fn init_master(&self) {
        unsafe {
            ptr::write_volatile(self.spcr, 0b01010001);

            let _ = ptr::read_volatile(self.spsr);
            let _ = ptr::read_volatile(self.spdr);
        }
    }

    /// Transmettre et recevoir une donnée via SPI
    pub fn spi_transfer(&self, data: u8) -> u8 {
        unsafe {
            ptr::write_volatile(self.spdr, data);

            while ptr::read_volatile(self.spsr) & (1 << 7) == 0 {}

            ptr::read_volatile(self.spdr)
        }
    }
}
