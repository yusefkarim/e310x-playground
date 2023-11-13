#![no_std]
#![no_main]

use common::*;
use core::{arch::asm, fmt::Write};
use e310x as device;

#[riscv_rt::entry]
fn main() -> ! {
    // rtt_target::rtt_init_default!();
    // rtt_target::rprintln!("Hello, RISC-V!");
    // defmt::info!("Hello, RISC-V!");
    let mut output = jlink_rtt::Output::new();
    output.write_str("Hello, RISC-V!").unwrap();

    // Take ownership of the device peripherals singleton
    if let Some(dp) = device::Peripherals::take() {
        let gpio = dp.GPIO0;
        // Blue LED
        gpio.input_en.modify(|_, w| w.pin5().clear_bit());
        gpio.drive.modify(|_, w| w.pin5().clear_bit());
        gpio.out_xor.modify(|_, w| w.pin5().clear_bit());
        gpio.output_en.modify(|_, w| w.pin5().set_bit());
        gpio.iof_en.modify(|_, w| w.pin5().clear_bit());

        loop {
            if gpio.output_val.read().pin5().bit_is_clear() {
                gpio.output_val.modify(|_, w| w.pin5().set_bit()); // ON
            } else {
                gpio.output_val.modify(|_, w| w.pin5().clear_bit()); // OFF
            }
            for _ in 0..10_000_00 {
                unsafe {
                    asm!("nop");
                }
            }
        }
    };
    panic!();
}
