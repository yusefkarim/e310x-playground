#![no_std]

use e310x as device;

/// Use the 13.8 MHz internal trimmable high-frequency ring oscillator (HFROSC) output for coreclk
pub fn use_13p8mhz_hfrosc_clock(dp: &device::Peripherals) {
    let prci = &dp.PRCI;

    // Configure HFROSC to 13.8 HMz. It is 13.8 MHz by default on reset, this is just for demonstration purposes.
    // The output frequency is 72 MHz, the value in hfroscdiv divides the clock by hfroscdiv + 1.
    // Not quite sure how trim affects the frequency, but higher trim values means higher frequencies, 16 is in the middle.
    prci.hfrosccfg
        .modify(|_, w| unsafe { w.div().bits(4).trim().bits(16).enable().set_bit() });

    // Wait for HFROSC to stabilize
    while !prci.hfrosccfg.read().ready().bit_is_set() {}
}
