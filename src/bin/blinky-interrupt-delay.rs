//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

use e310x_hal as hal;
use e310x_playground as _;
use hal::{clock::Clocks, delay::Sleep, prelude::*, DeviceResources};
use rtt_target;

#[riscv_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!(NoBlockSkip);
    rtt_target::rprintln!("System starting up...");

    if let Some(dr) = DeviceResources::take() {
        rtt_target::rprintln!("Configuring peripherals...");
        let clint = dr.core_peripherals.clint;
        let prci = dr.peripherals.PRCI.constrain();
        let aonclk = dr.peripherals.AONCLK.constrain();

        // Configure the core clock frequency, running it at 16 MHz.
        // Uses the HFXOSC (external oscillator) instead of the
        // HFROSC (internal ring oscillator) as the clock source.
        let coreclk = prci.use_external(16.mhz()).coreclk(16.mhz());
        // Configure the Always-On (AON) Domain clock frequency, running it at 32768 Hz.
        // Uses the LFALTCLK (external low-frequency clock) instead of the
        // LFROSC (internal ring oscillator) as the clock source.
        let aonclk = aonclk.use_external(32_768.hz());
        let clocks = Clocks::freeze(coreclk, aonclk);
        // Configure the LED on GPIO0 pin 5 as an output.
        let mut led = dr.pins.pin5.into_output();
        // Create a delay abstraction based on the MTIMECMP register
        // which uses an internal timer interrupt under the hood.
        let mut sleep = Sleep::new(clint.mtimecmp, clocks);

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
