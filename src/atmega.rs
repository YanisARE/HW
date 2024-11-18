#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]


mod gpio;
mod usart;

use gpio::{GpioPin, PinMode};
use usart::Usart;

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

    output_pin.write(false);

    // Boucle principale
    loop {
        output_pin.write(false);  // Allume la LED
        delay_ms(500);           // Attente de 500 ms
        output_pin.write(true); // Éteint la LED
        delay_ms(500);

        usart.transmit(b'H'); // Envoie le caractère 'H'
        delay_ms(100);
        usart.transmit(b'i');
        delay_ms(3000);

        let received: u8 = usart.receive().unwrap_or(0); // 0 est la valeur par défaut en cas de timeout
        delay_ms(200);
        usart.transmit(received);

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
