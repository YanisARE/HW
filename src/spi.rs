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
            // Configurer le registre SPCR pour activer SPI, mode Master, prescaler = 16
            ptr::write_volatile(self.spcr, 0b01010001); // SPE=1, MSTR=1, SPR1:SPR0=01
        }
    }

    /// Transmettre une donnée via SPI
    pub fn transmit(&self, data: u8) {
        unsafe {
            // Écrire dans SPDR pour transmettre les données
            ptr::write_volatile(self.spdr, data);

            // Attendre que la transmission soit terminée
            while ptr::read_volatile(self.spsr) & 0x80 == 0 {}
        }
    }

    /// Recevoir une donnée via SPI
    pub fn receive(&self) -> u8 {
        unsafe {
            // Transmettre un octet vide pour générer l'horloge
            self.transmit(0x00);

            // Lire les données reçues dans SPDR
            ptr::read_volatile(self.spdr)
        }
    }
}
