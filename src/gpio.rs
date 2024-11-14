pub struct GpioPin {
    port: *mut u8,
    ddr: *mut u8,
    pin: u8,
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
