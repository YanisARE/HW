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
    hprintln!("Hello, world!\n").ok();  // .ok() permet de traiter le résultat de hprintln sans erreur.

    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    let output_pin = GpioPin::new(PORTB, DDRB, 2, PinMode::Output);
    let input_pin_pullup = GpioPin::new(PORTB, DDRB, 3, PinMode::InputPullUp);

    output_pin.write(true);

    let mut last_state = input_pin_pullup.read();

    loop {
        let input_state = input_pin_pullup.read();

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

        // Modifier l'état de la broche de sortie en fonction de l'entrée
        if input_state {
            output_pin.write(false); // Désactive la sortie si l'entrée est à 1
        } else {
            output_pin.write(true);  // Active la sortie si l'entrée est à 0
        }
    }
}
