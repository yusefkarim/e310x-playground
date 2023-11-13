#![no_std]
#![no_main]

use common::*;
use e310x_hal::{clock::Clocks, delay::Sleep, e310x as pac, prelude::*, time::Hertz, DeviceResources};

#[riscv_rt::entry]
fn main() -> ! {
    // Take ownership of the device resource & peripheral singletons
    if let (Some(dr), Some(dp)) = (DeviceResources::take(), pac::Peripherals::take()) {
        let clint = dr.core_peripherals.clint;
        let aon = dp.AONCLK;
        let prci = dp.PRCI;

        let coreclk = prci.constrain();
        let coreclk = coreclk
            .use_external(Hertz(16_000_000))
            .coreclk(Hertz(8_000_000));
        let aonclk = aon.constrain();
        let aonclk = aonclk.use_external(Hertz(32_768));
        let clocks = Clocks::freeze(coreclk, aonclk);

        let mut sleep = Sleep::new(clint.mtimecmp, clocks);

        let gpio = dp.GPIO0.split();
        let mut led = gpio.pin5.into_output();

        loop {
            led.set_high().unwrap();
            sleep.delay_ms(500);
            led.set_low().unwrap();
            sleep.delay_ms(500);
        }

    };
    panic!();
}
