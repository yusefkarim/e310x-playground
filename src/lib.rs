#![no_std]

use core::{arch::asm, panic::PanicInfo};

// Install a global panic handler that uses RTT
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Disable interrupts
    unsafe { asm!("csrci mstatus, 8") };
    rtt_target::debug_rprintln!("In panic handler: {}", info);
    loop {}
}

// Exports
// pub use riscv;
// pub use riscv_rt;
// pub use rtt_target;
// pub use defmt;
// pub use jlink_rtt;
