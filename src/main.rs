#![no_std]
#![no_main]

mod gpio;
use gpio::{GpioPin, PinMode};

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    hprintln!("Welcome to our Rust blink program!\n").ok();

    // Définitions des registres pour le PORTB et DDRB
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    // Initialisation de la broche 2 en mode sortie
    let output_pin = GpioPin::new(PORTB, DDRB, 2, PinMode::Output);

    output_pin.write(false);

    loop {

        let mut state = output_pin.read();
        if state  {
            hprintln!("LED is ON").ok();
            delay_ms(1000);
            output_pin.write(false);
        }
        else {
            hprintln!("LED is OFF").ok();
            delay_ms(1000);
            output_pin.write(true);
        }
    }
}

// Fonction simple de temporisation (peut varier selon l'environnement)
fn delay_ms(ms: u32) {
    for _ in 0..ms * 16_000 { // Boucle approximative pour 1 ms à 16 MHz
        // NOP ou simplement laisser la boucle vide
        unsafe { core::arch::asm!("nop") }
    }
}
