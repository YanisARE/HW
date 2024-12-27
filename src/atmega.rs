#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

mod gpio;
mod usart; // import the USART module
mod spi;
mod i2c; // import the I2C module
// import the SPI module

use gpio::{GpioPin, PinMode, SpiPins}; // add SpiPins
use usart::Usart; // import the Usart structure
use spi::Spi;     // import the Spi structure
use i2c::I2c;     // import the I2C structure

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub extern "C" fn main() -> ! {
    const PORTB: *mut u8 = 0x25 as *mut u8; // address of the PORTB register
    const DDRB: *mut u8 = 0x24 as *mut u8;  // address of the DDRB register

    // initialize SPI pins
    let spi_pins = SpiPins::new(PORTB, DDRB);

    // initialize USART with a baud rate of 9600 (UBRR = 103 for 16 MHz)
    let usart = Usart::new(103);

    // initialize SPI in master mode
    let spi = Spi::new();
    spi.init_master();

    // initialize I2C
    let i2c = I2c::new();
    i2c.init(); // initialize I2C

    spi_pins.reset(); // reset SPI pins

    loop {
        // SPI test: transmit and receive
        spi_pins.set_ss(false); // activate the SPI slave
        delay_ms(50);

        // transmit data and wait for the response
        let received_data = spi.spi_transfer(b'H');

        // check if the received data is valid
        if received_data != 0xFF {
            usart.transmit(received_data);
        } else {
            usart.transmit(b'E');
        }

        delay_ms(50);
        spi_pins.set_ss(true); // deactivate the SPI slave

        // send a character via USART for verification
        usart.transmit(b'H');
        delay_ms(1000); // delay 1 second before the next iteration

        // I2C test
        if test_i2c(&i2c) {
            usart.transmit(b'I'); // if I2C test is successful, transmit 'I'
        } else {
            usart.transmit(b'E'); // if test fails, transmit 'E'
        }

        delay_ms(1000); // delay 1 second before the next I2C test
    }
}

/// approximate delay in milliseconds (based on F_CPU of 16 MHz)
fn delay_ms(ms: u16) {
    for _ in 0..ms {
        for _ in 0..1000 {
            unsafe { core::arch::asm!("nop") }; // NOP to slow down execution
        }
    }
}

/// I2C test function: tries to write and read data
fn test_i2c(i2c: &I2c) -> bool {
    const I2C_SLAVE_ADDR: u8 = 0x50; // fake I2C slave address (to be adjusted)

    // try writing data (e.g., 0x55) to the I2C slave
    i2c.write(I2C_SLAVE_ADDR, 0x55);

    // try reading data from the I2C slave
    let received_data = i2c.read(I2C_SLAVE_ADDR);

    // check if the received data matches the sent data
    received_data == 0x55
}
