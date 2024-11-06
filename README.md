# Launching the project on your machine

You first have to build the project with the next command (you may have installed nightly):

cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release

You should now have a .elf file in target/avr-unknown-gnu-atmega328/release

Finally, we can execute this code with qemu:

qemu-system-avr -machine uno -bios target/avr-unknown-gnu-atmega328/release/simple_blink.elf -s -S

Then in another shell with gdb : 

avr-gdb simple_blink.elf

target remote localhost:1234

break main

continue


test



