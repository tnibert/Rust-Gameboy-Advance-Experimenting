#! /bin/bash
cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/agb_play test.gba
gbafix test.gba