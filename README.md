# Launching the project on your machine

 - For the atmega, you can juste download the ouput.hex file if you want to execute the code, but if you want to modify the code and test it by yourself, you'll have to follow the next steps.
 - For the thumbv7em-none-eabihf, just run the 2 commands below

## Arduino Uno (Atmega328p)

You first have to build the project with the next command (you may have installed nightly):

```
cargo +nightly build -Z build-std=core --target avr-atmega328p.json --release --bin atmega
/mnt/c/avr/avr-gcc-14.1.0-x64-windows/bin/avr-gcc.exe -mmcu=atmega328 -o output.elf ./target/avr-atmega328p/release/deps/*.o ./target/avr-atmega328p/release/deps/*.rlib
/mnt/c/avr/avr-gcc-14.1.0-x64-windows/bin/avr-objcopy.exe -O ihex ./output.elf output.hex
C:\avr\avr-gcc-14.1.0-x64-windows\bin\avrdude.exe -C C:\avr\avr-gcc-14.1.0-x64-windows\bin\avrdude.conf -v -patmega328p -carduino -PCOM3 -b115200 -Uflash:w:output.hex:i
```

With the last command you are flashing a physical Atmega328p card, but if you don't have one, you can execute this code with qemu (next part).

Now, using Putty or another tool, you can see that the card transmit "Hi" to us, then we send data to it (you have 3 seconds to send data), it receives the data and transmit it back to us:

![img_1.png](img_1.png)

### With qemu 
```

```

## thumbv7em-none-eabihf


```
cargo build --target thumbv7em-none-eabihf --bin thumbv7
socat -d -d pty,raw,echo=0 pty,raw,echo=0
cargo run --bin thumbv7
putty
```
