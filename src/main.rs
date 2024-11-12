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
    hprintln!("Hello, world!\n").ok();  // Affiche un message de démarrage

    // Définitions des registres pour le PORTB et DDRB
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    // Initialisation de la broche 2 en mode sortie
    let output_pin = GpioPin::new(PORTB, DDRB, 2, PinMode::Output);
    // Initialisation de la broche 3 en mode entrée avec pull-up
    let input_pin_pullup = GpioPin::new(PORTB, DDRB, 3, PinMode::InputPullUp);

    // Mise à 1 de la broche de sortie
    output_pin.write(true);

    let mut last_state = input_pin_pullup.read();
    hprintln!("Initial input state: {:?}", last_state).ok(); // Affiche l'état initial

    loop {
        let input_state = input_pin_pullup.read();
        hprintln!("Current input state: {:?}", input_state).ok(); // Affiche l'état actuel

        // Vérifier si l'état a changé
        if input_state != last_state {
            // Si l'état a changé, écrivez un message
            if input_state {
                hprintln!("Pin is HIGH\n").ok();
            } else {
                hprintln!("Pin is LOW\n").ok();
            }

            // Mettre à jour l'état précédent
            last_state = input_state;
        }

        if input_state {
            output_pin.write(false);
        } else {
            output_pin.write(true);
        }

        // Petite temporisation pour éviter une boucle trop rapide
        for _ in 0..1_000_000 {}
    }
}
