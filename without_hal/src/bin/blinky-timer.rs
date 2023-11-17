#![no_std]
#![no_main]

use common::*;
use device::interrupt;
use e310x as device;
use riscv::asm::wfi;
use riscv::register::{mie, mip};
use without_hal::use_13p8mhz_hfrosc_clock;

static CLOCK_FREQ: u32 = 13_800_000;

#[riscv_rt::entry]
fn main() -> ! {
    // Take ownership of the device peripherals singleton
    if let Some(dp) = device::Peripherals::take() {
        use_13p8mhz_hfrosc_clock(&dp);
        let gpio = dp.GPIO0;
        let clint = dp.CLINT;

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
            delay_ms(&clint, 1000);
        }
    };
    panic!();
}

fn periodic() {
    //
}

/// NOTE: DOES NOT WORK
/// Likely because mtime.read().bits() returns 32 bits, when in reality
/// mtime contains a 64 bit number
fn delay_ms(clint: &device::CLINT, ms: u32) {
    let ticks = ms * CLOCK_FREQ / 1000;
    // mtime is a 64-bit read-write register that contains the
    // number of cycles counted from the rtcclk input
    let t = clint.mtime.read().bits() + ticks;

    unsafe {
        clint.mtimecmp.write(|w| w.bits(t));
        mie::set_mtimer();
    }

    loop {
        unsafe {
            wfi();
        }

        if mip::read().mtimer() {
            break;
        }
    }

    unsafe {
        mie::clear_mtimer();
    }
}
