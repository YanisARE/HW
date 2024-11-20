#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

mod gpio;
mod usart; // Importer le module USART
mod spi;   // Importer le module SPI

use gpio::{GpioPin, PinMode, SpiPins}; // Ajout de SpiPins
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

    // Initialisation de la broche 5 (PB5) en mode sortie
    let output_pin = GpioPin::new(PORTB, DDRB, 5, PinMode::Output);

    // Initialisation des broches SPI
    let spi_pins = SpiPins::new(PORTB, DDRB);

    // Initialisation de l'USART avec une vitesse de 9600 bauds (UBRR = 103 pour 16 MHz)
    let usart = Usart::new(103);

    // Initialisation de SPI en mode maître
    let spi = Spi::new();
    spi.init_master();

    // Réinitialiser les broches SPI
    spi_pins.reset();

    // Boucle principale
    loop {
        // Test GPIO : clignotement de la LED sur PB5
        output_pin.write(false); // Allume la LED
        delay_ms(500);           // Attente de 500 ms
        output_pin.write(true);  // Éteint la LED
        delay_ms(500);

        // Test USART : envoi de caractères
        usart.transmit(b'H'); // Envoie le caractère 'H'
        delay_ms(10);         // Pause pour éviter un conflit matériel
        usart.transmit(b'i'); // Envoie le caractère 'i'

        // Réception via USART
        if let Some(received) = usart.receive(400) {
            usart.transmit(received); // Renvoie la donnée reçue
        } else {
            usart.transmit(b'x'); // Envoie 'x' si aucune donnée reçue
        }

        // Test SPI : transmission et réception
        //spi_pins.set_ss(true); // Activer l'esclave SPI
        //let data_to_send = 0x55; // Exemple de données à transmettre
        //spi.transmit(data_to_send); // Transmettez une donnée
        //let spi_received = spi.receive(); // Recevez la réponse
        //spi_pins.set_ss(false); // Désactiver l'esclave SPI

        // Envoyer les données SPI reçues via USART pour vérification
        //usart.transmit(spi_received);

        // Pause avant la prochaine itération
        delay_ms(200);
    }
}

/// Délai approximatif en millisecondes (basé sur un F_CPU de 16 MHz)
fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..1000 {
            unsafe { core::arch::asm!("nop") }; // NOP pour ralentir l'exécution
        }
    }
}
