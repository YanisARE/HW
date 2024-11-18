pub struct Usart {
    ubrrh: *mut u8,
    ubrrl: *mut u8,
    ucsra: *mut u8,
    ucsrb: *mut u8,
    udr: *mut u8,
}

impl Usart {
    /// Initialise l'USART avec une vitesse de transmission donnée
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
    pub fn receive(&self) -> Option<u8> {


        unsafe {
            while (*self.ucsra & (1 << 7)) == 0 {

            }
            Some(*self.udr) // Donnée reçue
        }
    }


}