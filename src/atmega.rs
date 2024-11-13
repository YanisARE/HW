#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]


mod gpio;
use gpio::{GpioPin, PinMode};


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main()
    -> ! {

    // Définitions des registres pour le PORTB et DDRB
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Adresse du registre DDRB

    // Initialisation de la broche 2 en mode sortie
    let output_pin = GpioPin::new(PORTB, DDRB, 5, PinMode::Output);

    output_pin.write(false);

    loop {

        let mut state = output_pin.read();
        if state  {
            delay_ms(100);
            output_pin.write(false);
        }
        else {
            delay_ms(100);
            output_pin.write(true);
        }
    }
}



fn delay_ms(ms: u32) {
    for _ in 0..ms * 16_000 { // Boucle approximative pour 1 ms à 16 MHz
        // NOP ou simplement laisser la boucle vide
        unsafe { core::arch::asm!("nop") }
    }
}

