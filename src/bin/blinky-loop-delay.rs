//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

use e310x_hal as hal;
use e310x_playground as _;
use hal::{delay::Delay, prelude::*, DeviceResources};
use rtt_target;

#[riscv_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!(NoBlockSkip);
    rtt_target::rprintln!("System starting up...");

    if let Some(dr) = DeviceResources::take() {
        rtt_target::rprintln!("Configuring peripherals...");

        // Configure the LED on GPIO0 pin 5 as an output.
        let mut led = dr.pins.pin5.into_output();
        // Create a delay abstraction based on the MTIME register which uses a
        // busy loop for checking the value loaded into MTIME under the hood.
        let mut sleep = Delay::new();

        rtt_target::rprintln!("Entering main loop...");
        loop {
            // On for 500ms, off for 500ms.
            led.toggle().unwrap();
            sleep.delay_ms(500_u32);
            rtt_target::rprint!(".");
        }
    }
    panic!();
}
