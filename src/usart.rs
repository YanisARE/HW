// usart.rs
#[cfg(target_arch = "avr")] // specific to AVR architecture
pub struct Usart {
    ubrrh: *mut u8,
    ubrrl: *mut u8,
    ucsra: *mut u8,
    ucsrb: *mut u8,
    udr: *mut u8,
}

#[cfg(target_arch = "avr")]
impl Usart {
    /// initialize the USART with a given baud rate (ATmega328P)
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
            *usart.ucsrb |= (1 << 3) | (1 << 4); // enable transmission and reception
        }

        usart
    }

    /// transmit a byte via USART
    pub fn transmit(&self, data: u8) {
        unsafe {
            // wait until the data register is empty
            while (*self.ucsra & (1 << 5)) == 0 {} // wait for UDRE0 bit

            // transmit the data
            *self.udr = data;
        }
    }

    /// receive a byte via USART
    pub fn receive(&self, timeout_ms: u16) -> Option<u8> {
        let mut elapsed = 0;

        unsafe {
            // wait until data is available or timeout is reached
            while (*self.ucsra & (1 << 7)) == 0 {
                delay_ms(10); // wait 1 ms
                elapsed += 1;

                if elapsed >= timeout_ms {
                    return None; // return `None` if timeout is reached
                }
            }

            // if data is available, read and return it
            Some(*self.udr)
        }
    }
}

#[cfg(target_arch = "arm")] // specific to ARM architecture (STM32 or other Cortex-M)
pub struct Usart {
    usart_base: *mut u32,  // base address for USART
}

#[cfg(target_arch = "arm")]
impl Usart {
    /// initialize the USART with a given baud rate (for ARM Cortex-M)
    pub fn new(ubrr_value: u16) -> Self {
        let usart = Usart {
            usart_base: 0x40011000 as *mut u32, // example: base address of USART1 on STM32F4
        };

        unsafe {
            let cr1 = usart.usart_base.offset(0x0C / 4); // correct offset for CR1 register
            let brr = usart.usart_base.offset(0x08 / 4); // correct offset for BRR register

            // configure the BRR register for baud rate
            *brr = ubrr_value as u32;

            // configure bits to enable reception and transmission
            *cr1 |= (1 << 13); // UE: USART Enable
            *cr1 |= (1 << 3);  // TE: Transmitter Enable
            *cr1 |= (1 << 2);  // RE: Receiver Enable
        }

        usart
    }

    /// transmit a byte via USART
    pub fn transmit(&self, data: u8) {
        unsafe {
            let sr = self.usart_base.offset(0x00 / 4); // offset for SR register
            let dr = self.usart_base.offset(0x04 / 4); // offset for DR register

            // wait until the data register is empty (TXE)
            while (*sr & (1 << 7)) == 0 {} // check TXE bit (Transmitter Empty)

            // transmit the data
            *dr = data as u32;
        }
    }

    /// receive a byte via USART
    pub fn receive(&self, timeout_ms: u16) -> Option<u8> {
        let mut elapsed = 0;

        unsafe {
            let sr = self.usart_base.offset(0x00 / 4); // correct offset for SR register
            let dr = self.usart_base.offset(0x04 / 4); // correct offset for DR register

            // wait until data is available or timeout is reached
            while (*sr & (1 << 5)) == 0 {  // check RXNE bit (Receiver Not Empty)
                delay_ms(10); // wait 1 ms
                elapsed += 1;

                if elapsed >= timeout_ms {
                    return None; // return `None` if timeout is reached
                }
            }

            // read the available data and return it
            Some(*dr as u8)
        }
    }
}

fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..1000 {
            unsafe { core::arch::asm!("nop") }; // NOP to slow down execution
        }
    }
}
