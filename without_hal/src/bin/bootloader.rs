#![no_std]
#![no_main]
/// An extremely simply bootloader
///
/// Somewhat modeled off of: https://github.com/jamolnng/hifive1b_bootloader/blob/master/main.c
///
/// - Checks for the magic value `0xCAFEE000` (stored in `AON_BACKUP15` register which survives resets/sleeps).
///   If it's there, it just jumps to the main firmware (address `0x20010000`).
///   This is meant to be used when you configure and use the sleep mechanism in your firmware, and want to skip the rest of the bootloader behaviour on wakeup.
///   (If something goes wrong, disconnect the board from power and wait a bit. This will zero out the magic register.)
/// 
/// - Checks for the magic value `0xD027B000`.
///   If it's there, it means (presumably) that reset was "double tapped" (see below),
///   and so the bootloader will just start blinking the onboard (on Red-V Thing) LED forever.
///   You will (should?) be able to then use JLink to connect and/or flash new firmware.
///   Note that magic (`AON_BACKUP15`) is set to `0`: so if you reset now, the bootloader will go to the default behaviour, described in the next option:
/// 
/// - None of the above. This is what happens by default, and also on powerup (but see the note below).
///   Turn on the onboard LED for 1/2 second. If you hit reset within this window, the "double tap" behaviour described above is triggered.
///   If you don't do anything, the LED is turned off, the original `AON_BACKUP15` value is restored, and the bootloader jumps to the main firmware.
/// 
/// NOTE: this bootloader should be flashed first at 0x20000000, while all subsequent firmware should be flashed at 0x20010000.

use core::arch::asm;

use common::*;
use e310x as device;

const USER_PROGRAM_START_ADDRESS: u32 = 0x20010000;
const BACKUP15_MAGIC: u32 = 0xD027B000;
const BACKUP15_MAGIC_BYPASS: u32 = 0xCAFEE000;

macro_rules! gpio_as_output {
    ($gpio: tt, $pin: ident) => {
        $gpio.input_en.modify(|_, w| w.$pin().clear_bit());
        $gpio.drive.modify(|_, w| w.$pin().clear_bit());
        $gpio.out_xor.modify(|_, w| w.$pin().clear_bit());
        $gpio.output_en.modify(|_, w| w.$pin().set_bit());
        $gpio.iof_en.modify(|_, w| w.$pin().clear_bit());
    };
}

#[riscv_rt::entry]
fn main() -> ! {
    unsafe {
        // Set interrupt trap vector to 0.
        // By default, this would cause an infinite loop upon exception, which is
        // also "safe" behavior and the debugger can connect.
        riscv::register::mtvec::write(0x0, riscv::register::mtvec::TrapMode::Direct);
    }

    // Take ownership of the device peripherals singleton
    if let Some(dp) = device::Peripherals::take() {
        let gpio = dp.GPIO0;
        let aon = dp.AON;

        // Check the backup register for magic value(s)
        if aon.backup_15.read().bits() == BACKUP15_MAGIC_BYPASS {
            unsafe { jump_to_user_program() }
        }

        // Set blue user LED (GPIO0 pin 5) as output
        gpio_as_output!(gpio, pin5);

        // Check for double tap of the RESET button
        if aon.backup_15.read().bits() == BACKUP15_MAGIC {
            // Zero the backup reg, so that the next reset goes back to the user app
            aon.backup_15.write(|w| unsafe { w.bits(0x0) });
        }

        // Blink LED forever
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

// #[inline]
unsafe fn jump_to_user_program() -> ! {
    reset_regs();
    asm!(
        "jr {addr}",
        addr = in(reg) USER_PROGRAM_START_ADDRESS,
        options(noreturn, nomem, nostack),
    );
}

#[inline]
unsafe fn reset_regs() {
    asm!(
        "csrw mie, 0",
        "csrw mip, 0",
        "li	sp, 0",
        "li	gp, 0",
        "li	tp, 0",
        "li	t0, 0",
        "li	t1, 0",
        "li	t2, 0",
        "li	s0, 0",
        "li	s1, 0",
        // a0..a2 (x10..x12) skipped
        "li	a2, 0",
        "li	a3, 0",
        "li	a4, 0",
        "li	a5, 0",
        "li	a6, 0",
        "li	a7, 0",
        "li	s2, 0",
        "li	s3, 0",
        "li	s4, 0",
        "li	s5, 0",
        "li	s6, 0",
        "li	s7, 0",
        "li	s8, 0",
        "li	s9, 0",
        "li	s10, 0",
        "li	s11, 0",
        "li	t3, 0",
        "li	t4, 0",
        "li	t5, 0",
        "li	t6, 0",
        options(nomem, nostack),
    );
}
