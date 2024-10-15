# Makefile for Rust AVR project

# Variables
TARGET = avr-unknown-gnu-atmega328
RELEASE_BUILD = target/$(TARGET)/release/blinky
SRC_DIR = src
BUILD_DIR = target

# Rust and Xargo tools
CARGO = cargo
XARGO = xargo
RUSTUP = rustup
VALGRIND = valgrind

# Default target: Build the project
all: build

# Build the project using Xargo for AVR target
build:
	$(XARGO) build --target $(TARGET) --release

# Clean the build directory
clean:
	$(CARGO) clean

# Flash the built project to the microcontroller (optional - adjust based on setup)
flash:
	avrdude -v -c arduino -p m328p -P /dev/ttyACM0 -b 115200 -U flash:w:$(RELEASE_BUILD).hex:i

# Run tests (if applicable)
test:
	$(CARGO) test

# Format the code using rustfmt
format:
	$(CARGO) fmt

# Check for memory leaks (only for specific targets like native builds)
valgrind:
	$(VALGRIND) ./$(RELEASE_BUILD)

# Set the Rust nightly toolchain and target
setup:
	$(RUSTUP) toolchain install nightly
	$(RUSTUP) target add $(TARGET) --toolchain nightly
	$(RUSTUP) override set nightly

# Phony targets
.PHONY: all build clean flash test format setup valgrind
