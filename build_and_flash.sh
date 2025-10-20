#!/bin/bash
cargo b --example ocarina --release && avrdude -c arduino -p m328p -b 57600 -P /dev/arduino/nano -U flash:w:target/avr-atmega328p/release/examples/ocarina.elf
