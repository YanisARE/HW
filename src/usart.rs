#[cfg(target_arch = "avr")] // Spécifique à l'architecture AVR
pub struct Usart {
    ubrrh: *mut u8,
    ubrrl: *mut u8,
    ucsra: *mut u8,
    ucsrb: *mut u8,
    udr: *mut u8,
}

#[cfg(target_arch = "avr")]
impl Usart {
    /// Initialise l'USART avec une vitesse de transmission donnée (ATmega328P)
    pub fn new(ubrr_value: u16) -> Self {
        let usart = Usart {
            ubrrh: 0xC5 as *mut u8, // UBRR0H
            ubrrl: 0xC4 as *mut u8, // UBRR0L
            ucsra: 0xC0 as *mut u8, // UCSR0A
            ucsrb: 0xC1 as *mut u8, // UCSR0B
            udr: 0xC6 as *mut u8,   // UDR0
        };

        unsafe {
            *usart.ubrrh = (ubrr_value >> 8) as u8;
            *usart.ubrrl = ubrr_value as u8;
            *usart.ucsrb |= (1 << 3) | (1 << 4); // Active la transmission et réception
        }

        usart
    }

    /// Transmet un octet via USART
    pub fn transmit(&self, data: u8) {
        unsafe {
            // Attente que le registre de données soit vide
            while (*self.ucsra & (1 << 5)) == 0 {} // Attente du bit UDRE0

            // Transmission des données
            *self.udr = data;
        }
    }

    /// Reçoit un octet via USART
    pub fn receive(&self, timeout_ms: u16) -> Option<u8> {
        let mut elapsed = 0;

        unsafe {
            // Attendre jusqu'à ce que des données soient disponibles ou jusqu'au dépassement du délai
            while (*self.ucsra & (1 << 7)) == 0 {
                delay_ms(10); // Attendre 1 ms
                elapsed += 1;

                if elapsed >= timeout_ms {
                    return None; // Retourne `None` si le délai est dépassé
                }
            }

            // Si des données sont disponibles, les lire et les retourner
            Some(*self.udr)
        }
    }
}

#[cfg(target_arch = "arm")] // Spécifique à l'architecture ARM (STM32 ou autre Cortex-M)
pub struct Usart {
    usart_base: *mut u32,  // Adresse de base pour l'USART
}

#[cfg(target_arch = "arm")]
impl Usart {
    /// Initialise l'USART avec une vitesse de transmission donnée (pour ARM Cortex-M)
    pub fn new(ubrr_value: u16) -> Self {
        let usart = Usart {
            usart_base: 0x40011000 as *mut u32, // Exemple : base address de USART1 sur STM32F4
        };

        unsafe {
            let cr1 = usart.usart_base.offset(0x0C / 4); // Offset correct pour le registre CR1
            let brr = usart.usart_base.offset(0x08 / 4); // Offset correct pour le registre BRR

            // Configurer le registre BRR pour le baud rate
            *brr = ubrr_value as u32;

            // Configuration des bits pour activer la réception et la transmission
            *cr1 |= (1 << 13); // UE: USART Enable
            *cr1 |= (1 << 3);  // TE: Transmitter Enable
            *cr1 |= (1 << 2);  // RE: Receiver Enable
        }

        usart
    }

    /// Transmet un octet via USART
    pub fn transmit(&self, data: u8) {
        unsafe {
            let sr = self.usart_base.offset(0x00 / 4); // Offset correct pour le registre SR
            let dr = self.usart_base.offset(0x04 / 4); // Offset correct pour le registre DR

            // Attente que le registre de données soit vide (TXE)
            while (*sr & (1 << 7)) == 0 {} // Vérifier le bit TXE (Transmitter Empty)

            // Transmission des données
            *dr = data as u32;
        }
    }

    /// Reçoit un octet via USART
    pub fn receive(&self, timeout_ms: u16) -> Option<u8> {
        let mut elapsed = 0;

        unsafe {
            let sr = self.usart_base.offset(0x00 / 4); // Offset correct pour le registre SR
            let dr = self.usart_base.offset(0x04 / 4); // Offset correct pour le registre DR

            // Attente jusqu'à ce que des données soient disponibles ou jusqu'au dépassement du délai
            while (*sr & (1 << 5)) == 0 {  // Vérifier le bit RXNE (Receiver Not Empty)
                delay_ms(10); // Attente de 1 ms
                elapsed += 1;

                if elapsed >= timeout_ms {
                    return None; // Retourne `None` si le délai est dépassé
                }
            }

            // Lire les données disponibles et les retourner
            Some(*dr as u8)
        }
    }
}


fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..1000 {
            unsafe { core::arch::asm!("nop") }; // NOP pour ralentir l'exécution
        }
    }
}
