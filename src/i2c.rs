use core::ptr;

pub struct I2c {
    twbr: *mut u8,   // TWI Bit Rate Register
    twcr: *mut u8,   // TWI Control Register
    twdr: *mut u8,   // TWI Data Register
    twsr: *mut u8,   // TWI Status Register
    twar: *mut u8,   // TWI Address Register
}

impl I2c {
    pub fn new() -> Self {
        I2c {
            twbr: 0xB8 as *mut u8,  // Adresse du registre TWBR
            twcr: 0xBC as *mut u8,  // Adresse du registre TWCR
            twdr: 0xBB as *mut u8,  // Adresse du registre TWDR
            twsr: 0xB9 as *mut u8,  // Adresse du registre TWSR
            twar: 0xBA as *mut u8,  // Adresse du registre TWAR
        }
    }

    /// Initialiser l'I2C (TWI)
    pub fn init(&self) {
        unsafe {
            ptr::write_volatile(self.twbr, 0x48);  // Baud rate: Calculé selon le F_CPU
            ptr::write_volatile(self.twcr, 0x04);  // Activer l'I2C (TWI)
        }
    }

    /// Envoi de données I2C
    pub fn write(&self, address: u8, data: u8) {
        unsafe {
            // Charger l'adresse de l'esclave avec le bit d'écriture
            ptr::write_volatile(self.twar, address << 1);

            // Transférer les données
            ptr::write_volatile(self.twdr, data);

            // Démarrer la transmission et attendre qu'elle soit terminée
            ptr::write_volatile(self.twcr, 0xA4);  // Démarrer la transmission (TWSTA = 1)
            while (ptr::read_volatile(self.twsr) & 0xF8) != 0x28 {}  // Attente du statut 'Transmission OK'
        }
    }

    /// Lecture de données via I2C
    pub fn read(&self, address: u8) -> u8 {
        unsafe {
            // Charger l'adresse de l'esclave avec le bit de lecture
            ptr::write_volatile(self.twar, (address << 1) | 0x01);

            // Démarrer la réception et attendre la fin de la transmission
            ptr::write_volatile(self.twcr, 0xA4);  // Démarrer la réception (TWSTA = 1)
            while (ptr::read_volatile(self.twsr) & 0xF8) != 0x40 {}  // Attente du statut 'Reception OK'

            // Lire et retourner la donnée reçue
            ptr::read_volatile(self.twdr)
        }
    }

    /// Vérifie l'état de l'I2C (utile pour la gestion des erreurs)
    pub fn check_status(&self) -> u8 {
        unsafe {
            ptr::read_volatile(self.twsr)  // Retourne le statut courant
        }
    }
}
