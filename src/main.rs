#![no_std]
#![no_main]

mod gpio;
mod usart; // Importer le module usart
use gpio::{GpioPin, PinMode};
use usart::Usart; // Importer la structure Usart

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    hprintln!("Welcome to our Rust program with USART!\n").ok();

    // Définitions des registres pour le PORTB et DDRB
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    // Initialisation de la broche 2 en mode sortie
    let output_pin = GpioPin::new(PORTB, DDRB, 2, PinMode::Output);
    output_pin.write(false);

    // Initialisation de l'USART avec une vitesse de transmission (par exemple 9600 baud)
    let usart = Usart::new(103); // 103 correspond à UBRR pour 9600 baud avec un Fosc de 16 MHz

    // Boucle principale
    loop {
        // Allume la LED et attend 1 seconde
        output_pin.write(true);
        hprintln!("LED is ON").ok();
        delay_ms(500);

        // Éteint la LED et attend 1 seconde
        output_pin.write(false);
        hprintln!("LED is OFF").ok();
        delay_ms(500);

        // Transmet un message via USART
        usart.transmit(b'H');
        delay_ms(100);
        hprintln!("Sent 'H' via USART").ok();
        usart.transmit(b'i');
        delay_ms(100);
        hprintln!("Sent 'i' via USART").ok();

        // Attendre 3 secondes
        delay_ms(3000);

        // Reçoit un octet via USART
        if let Some(received) = usart.receive(400) { // Attend pendant 400 ms
            hprintln!("Received: {}", received as char).ok();
            usart.transmit(received); // Renvoie les données reçues
        } else {
            usart.transmit(b'x'); // Si aucune donnée n'est reçue, renvoie 'x'
        }

        // Petite pause avant la prochaine itération
        delay_ms(200);
    }
}

fn delay_ms(ms: u32) {
    for _ in 0..ms * 16_000 { // Boucle approximative pour 1 ms à 16 MHz
        unsafe { core::arch::asm!("nop") }
    }
}
