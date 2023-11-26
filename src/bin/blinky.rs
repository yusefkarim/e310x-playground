#![no_std]
#![no_main]

use e310x_hal::{clock::Clocks, delay::Sleep, prelude::*, time::Hertz, DeviceResources};
use e310x_playground as _;
use rtt_target;

#[riscv_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!(NoBlockSkip);
    rtt_target::rprintln!("System starting up...");
    // Take ownership of the device resource & peripheral singletons
    if let Some(dr) = DeviceResources::take() {
        rtt_target::rprintln!("Configuring peripherals...");
        let clint = dr.core_peripherals.clint;
        let aon = dr.peripherals.AONCLK;
        let prci = dr.peripherals.PRCI;

        let coreclk = prci.constrain();
        let coreclk = coreclk
            .use_external(Hertz(16_000_000))
            .coreclk(Hertz(8_000_000));
        let aonclk = aon.constrain();
        let aonclk = aonclk.use_external(Hertz(32_768));
        let clocks = Clocks::freeze(coreclk, aonclk);

        let mut sleep = Sleep::new(clint.mtimecmp, clocks);
        let mut led = dr.pins.pin5.into_output();

        rtt_target::rprintln!("Entering main loop...");
        loop {
            led.toggle().unwrap();
            sleep.delay_ms(500);
            rtt_target::rprint!(".");
        }
    };
    panic!();
}
