#![no_std]
#![no_main]

mod gpio;
mod usart; // import the USART module
mod spi;   // import the SPI module
mod i2c;   // import the I2C module

use gpio::{GpioPin, PinMode};
use usart::Usart; // import the Usart structure
use spi::Spi;     // import the Spi structure
use i2c::I2c;     // import the I2C structure

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use crate::gpio::SpiPins;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    hprintln!("Welcome to our Rust program with USART, SPI, and I2C!\n").ok();

    // define the registers for PORTB and DDRB
    const PORTB: *mut u8 = 0x25 as *mut u8; // address of the PORTB register
    const DDRB: *mut u8 = 0x24 as *mut u8;  // address of the DDRB register

    // initialize pin 2 as output mode
    let output_pin = GpioPin::new(PORTB, DDRB, 2, PinMode::Output);
    output_pin.write(false);

    let spi_pins = SpiPins::new(PORTB, DDRB);

    // initialize SPI in master mode
    let spi = Spi::new();
    spi.init_master();

    // initialize USART with a baud rate (for example, 9600 baud)
    let usart = Usart::new(103); // 103 corresponds to UBRR for 9600 baud with a 16 MHz clock

    // initialize I2C
    let i2c = I2c::new();
    i2c.init();

    loop {
        let mut state;
        // turn on the LED and wait for 1 second
        output_pin.write(true);
        hprintln!("LED is ON").ok();
        delay_ms(200);

        // turn off the LED and wait for 1 second
        output_pin.write(false);
        hprintln!("LED is OFF").ok();
        delay_ms(200);

        hprintln!("Testing USART communication...").ok();
        delay_ms(100);

        // transmit a message via USART
        usart.transmit(b'H');
        delay_ms(100);
        hprintln!("Sent 'H' via USART").ok();
        usart.transmit(b'i');
        delay_ms(100);
        hprintln!("Sent 'i' via USART").ok();

        // wait for 3 seconds
        delay_ms(2500);

        // receive a byte via USART
        if let Some(received) = usart.receive(400) { // wait for 400 ms
            hprintln!("Received: {}", received as char).ok();
            usart.transmit(received); // echo received data
        } else {
            usart.transmit(b'x'); // if no data received, send 'x'
        }

        // test SPI communication every iteration
        hprintln!("Testing SPI communication...").ok();
        let data_to_send = 0x55; // example data to send
        spi.spi_transfer(data_to_send);

        // test I2C communication every iteration
        hprintln!("Testing I2C communication...").ok();
        let i2c_data = 0x42; // example data for I2C
        i2c.write(i2c_data, 0);

        // short pause before the next iteration
        delay_ms(200);
    }
}

fn delay_ms(ms: u32) {
    for _ in 0..ms * 16_000 { // approximate loop for 1 ms at 16 MHz
        unsafe { core::arch::asm!("nop") }
    }
}
