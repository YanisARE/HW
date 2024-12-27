#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

mod gpio;
mod usart; // Importer le module USART
mod spi;
mod i2c; // Importer le module I2C
// Importer le module SPI

use gpio::{GpioPin, PinMode, SpiPins}; // Ajout de SpiPins
use usart::Usart; // Importer la structure Usart
use spi::Spi;     // Importer la structure Spi
use i2c::I2c;     // Importer la structure I2C

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub extern "C" fn main() -> ! {
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    // Initialisation des broches SPI
    let spi_pins = SpiPins::new(PORTB, DDRB);

    // Initialisation de l'USART avec une vitesse de 9600 bauds (UBRR = 103 pour 16 MHz)
    let usart = Usart::new(103);

    // Initialisation de SPI en mode maître
    let spi = Spi::new();
    spi.init_master();

    // Initialisation de l'I2C
    let i2c = I2c::new();
    i2c.init(); // Initialise l'I2C

    spi_pins.reset(); // Réinitialisation des broches SPI

    loop {
        // Test SPI : transmission et réception
        spi_pins.set_ss(false); // Activer l'esclave SPI
        delay_ms(50);

        // Transmettre une donnée et attendre la réponse
        let received_data = spi.spi_transfer(b'H');

        // Vérifier que la donnée reçue est valide
        if received_data != 0xFF {
            usart.transmit(received_data);
        } else {
            usart.transmit(b'E');
        }

        delay_ms(50);
        spi_pins.set_ss(true); // Désactiver l'esclave SPI

        // Envoi d'un caractère via USART pour vérification
        usart.transmit(b'H');
        delay_ms(1000); // Délai de 1 seconde avant la prochaine itération

        // Test I2C
        if test_i2c(&i2c) {
            usart.transmit(b'I'); // Si le test I2C est réussi, transmettre 'I'
        } else {
            usart.transmit(b'E'); // Si le test échoue, transmettre 'E'
        }

        delay_ms(1000); // Délai de 1 seconde avant le prochain test I2C
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

/// Fonction de test de l'I2C : essaie d'écrire et de lire une donnée
fn test_i2c(i2c: &I2c) -> bool {
    const I2C_SLAVE_ADDR: u8 = 0x50; // Adresse fictive de l'esclave I2C (à adapter)

    // Essai d'écriture d'une donnée (par exemple, 0x55) sur l'esclave I2C
    i2c.write(I2C_SLAVE_ADDR, 0x55);

    // Essai de lecture d'une donnée de l'esclave I2C
    let received_data = i2c.read(I2C_SLAVE_ADDR);

    // Vérifier que la donnée reçue correspond à la donnée envoyée
    received_data == 0x55
}
