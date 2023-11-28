# Learning embedded systems with the SparkFun RED-V Thing Plus (SiFive Freedom E310-G002) and Rust!

This repository is primarily for learning and trying random things with the E310-G002 MCU via the SparkFun RED-V Thing Plus board.

## Development Board
* [Freedom E310-G002 Manual](https://sifive.cdn.prismic.io/sifive/034760b5-ac6a-4b1c-911c-f4148bb2c4a5_fe310-g002-v1p5.pdf)
* [SparkFun RED-V Thing Plus](https://www.sparkfun.com/products/15799)
    - [Hardware GitHub repo](https://github.com/sparkfun/SparkFun_Red-V_Thing_Plus)

## On-board buttons

The RED-V Thing Plus has two buttons. These are the:

* RESET button
    - Resets the FE310
    - A single tap of the RESET button will run the code loaded onto the FE310's QSPI flash
    - A quick double tap will put the FE310 into safe bootloader mode, allowing you to flash new code to the RED-V if you've managed to really mess things up
    - The pin is also broken out on the edge of the board. Adding a jumper wire from this pin to GND will reset the board as well.
* WAKE button
    - The WAKE button can be configured in software to wake the FE310 from deep sleep via the Always-On (AON) core

## Getting started
These are the high-level steps and requirements you need to run the examples in this repo.

Install Cargo related tooling:
```sh
cargo install probe-rs --features cli
```

Uploading code:
```sh
cargo run --bin ${TARGET}
# Or using shortcut (defined in .cargo/config.toml):
cargo rb ${TARGET}
```

## Resources

* [probe-rs](https://github.com/probe-rs/probe-rs)
* [defmt](https://github.com/knurling-rs/defmt)
* [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)
* [OpenOCD](http://openocd.org/)
* [The Embedded Rust Book](https://rust-embedded.github.io/book/)
<!-- * [flip-link](https://github.com/knurling-rs/flip-link) -->


<!-- ## Miscellaneous Commands

```sh
# Flash a program with OpenOCD, replace ${TARGET_BIN} with your binary
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg -c "program ${TARGET_BIN} reset exit 0x08000000"
# Create a raw binary from an ELF, replace ${TARGET_ELF} with your compiled Rust code
# ${TARGET_BIN} can be named whatever you like
cargo objcopy --bin ${TARGET_ELF} -- -O binary ${TARGET_BIN}
# Use OpenOCD to erase all flash memory on target board
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg -c "init; reset halt; stm32f4x mass_erase 0; exit"
# Use semi-hosting to see debug output, requires STlink debugger
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg -c "init; arm semihosting enable"
# Attach to running OpenOCD server via GDB
arm-none-eabi-gdb -q ${TARGET_ELF} -ex "target remote localhost:3333"
``` -->
