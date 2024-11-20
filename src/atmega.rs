#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]


mod gpio;
mod usart; // Importer le module USART
mod spi;   // Importer le module SPI

use gpio::{GpioPin, PinMode};
use usart::Usart; // Importer la structure Usart
use spi::Spi;     // Importer la structure Spi
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Définition des registres pour PORTB et DDRB sur Atmega328p
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    // Initialisation de la broche 2 en mode sortie
    let output_pin = GpioPin::new(PORTB, DDRB, 5, PinMode::Output);

    // Initialisation de l'USART avec une vitesse de 9600 bauds (UBRR = 103 pour 16 MHz)
    let usart = Usart::new(103);
    let spi = Spi::new();
    spi.init_master();
    output_pin.write(false);

    // Boucle principale
    loop {
        output_pin.write(false);  // Allume la LED
        delay_ms(500);           // Attente de 500 ms
        output_pin.write(true); // Éteint la LED
        delay_ms(500);

        usart.transmit(b'H'); // Envoie le caractère 'H'
        delay_ms(500);
        usart.transmit(b'i');

        if let Some(received) = usart.receive(400) {
            usart.transmit(received);
        } else {
            usart.transmit(b'x');
        }

        let data_to_send = 0x55; // Exemple de données à transmettre
        spi.transmit(data_to_send);
        let spi_received = spi.receive();

        delay_ms(200);

    }
}



fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..1000 {
            unsafe { core::arch::asm!("nop") }; // Opération NOP pour ralentir l'exécution
        }
    }
}
