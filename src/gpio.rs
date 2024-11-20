pub struct GpioPin {
    port: *mut u8, // Pointeur vers le registre PORT
    ddr: *mut u8,  // Pointeur vers le registre DDR
    pin: u8,       // Numéro de la broche
}

pub enum PinMode {
    Input,
    Output,
}

impl GpioPin {
    /// Initialise une nouvelle broche GPIO avec le mode spécifié
    pub fn new(port: *mut u8, ddr: *mut u8, pin: u8, mode: PinMode) -> Self {
        let gpio_pin = GpioPin { port, ddr, pin };
        gpio_pin.set_mode(mode);
        gpio_pin
    }

    /// Configure le mode de la broche (entrée ou sortie)
    pub fn set_mode(&self, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => {
                    *self.ddr &= !(1 << self.pin); // Met la broche en entrée
                }
                PinMode::Output => {
                    *self.ddr |= 1 << self.pin; // Met la broche en sortie
                }
            }
        }
    }

    /// Écrit un niveau logique (haut/bas) sur la broche
    pub fn write(&self, high: bool) {
        unsafe {
            if high {
                *self.port |= 1 << self.pin; // État haut
            } else {
                *self.port &= !(1 << self.pin); // État bas
            }
        }
    }

    /// Lit l'état logique de la broche
    pub fn read(&self) -> bool {
        unsafe { (*self.port & (1 << self.pin)) != 0 }
    }
}

/// Une structure dédiée pour configurer les broches SPI
pub struct SpiPins {
    pub mosi: GpioPin,
    pub miso: GpioPin,
    pub sck: GpioPin,
    pub ss: GpioPin,
}

impl SpiPins {
    /// Initialise les broches SPI avec leurs rôles respectifs
    pub fn new(port: *mut u8, ddr: *mut u8) -> Self {
        SpiPins {
            mosi: GpioPin::new(port, ddr, 3, PinMode::Output), // MOSI (PB3) en sortie
            miso: GpioPin::new(port, ddr, 4, PinMode::Input),  // MISO (PB4) en entrée
            sck: GpioPin::new(port, ddr, 5, PinMode::Output),  // SCK (PB5) en sortie
            ss: GpioPin::new(port, ddr, 2, PinMode::Output),   // SS (PB2) en sortie
        }
    }

    /// Configure la broche SS pour activer ou désactiver l'esclave
    pub fn set_ss(&self, active: bool) {
        self.ss.write(!active); // Active = LOW (0), Inactive = HIGH (1)
    }
}
