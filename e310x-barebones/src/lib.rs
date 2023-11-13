#![no_std]
#![feature(naked_functions)]
#![feature(asm_sym)]

///
/// Naked functions: https://github.com/rust-lang/rfcs/blob/master/text/1201-naked-fns.md
/// asm_sym: TODO
use core::arch::asm;
use core::panic::PanicInfo;

extern "C" {
    // Where the end of the stack region is (and hence where the stack should
    // start).
    static _estack: usize;

    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;

    // The global pointer, value set in the linker script
    static __global_pointer: usize;
}

#[link_section = ".start"]
#[export_name = "_start"]
#[naked]
pub extern "C" fn _start() {
    unsafe {
        asm!("
        csrw mie, 0
        csrw mip, 0
        
        li  x1, 0
        li  x2, 0
        li  x3, 0
        li  x4, 0
        li  x5, 0
        li  x6, 0
        li  x7, 0
        li  x8, 0
        li  x9, 0
        li  x10,0
        li  x11,0
        li  x12,0
        li  x13,0
        li  x14,0
        li  x15,0
        li  x16,0
        li  x17,0
        li  x18,0
        li  x19,0
        li  x20,0
        li  x21,0
        li  x22,0
        li  x23,0
        li  x24,0
        li  x25,0
        li  x26,0
        li  x27,0
        li  x28,0
        li  x29,0
        li  x30,0
        li  x31,0
    
        // Set the global pointer register using the variable defined in the
        // linker script. This register is only set once. The global pointer
        // is a method for sharing state between the linker and the CPU so
        // that the linker can emit code with offsets that are relative to
        // the gp register, and the CPU can successfully execute them.
        lui  gp, %hi({gp}$)     // Set the global pointer.
        addi gp, gp, %lo({gp}$) // Value set in linker script.
    
        // Initialize the stack pointer register. This comes directly from
        // the linker script.
        lui  sp, %hi({estack})     // Set the initial stack pointer.
        addi sp, sp, %lo({estack}) // Value from the linker script.
    
        // Set s0 (the frame pointer) to the start of the stack.
        add  s0, sp, zero
    
        // INITIALIZE MEMORY
        //
        // Start by initializing .bss memory. The Tock linker script defines
        // `_szero` and `_ezero` to mark the .bss segment.
        la a0, {sbss}               // a0 = first address of .bss
        la a1, {ebss}               // a1 = first address after .bss
    
        bss_init_loop:
          beq  a0, a1, bss_init_done  // If a0 == a1, we are done.
          sw   zero, 0(a0)            // *a0 = 0. Write 0 to the memory location in a0.
          addi a0, a0, 4              // a0 = a0 + 4. Increment pointer to next word.
          j bss_init_loop             // Continue the loop.
      
        bss_init_done:
      
          // Now initialize .data memory. This involves coping the values right at the
          // end of the .text section (in flash) into the .data section (in RAM).
          la a0, {sdata}              // a0 = first address of data section in RAM
          la a1, {edata}              // a1 = first address after data section in RAM
          la a2, {etext}              // a2 = address of stored data initial values
      
        data_init_loop:
          beq  a0, a1, data_init_done // If we have reached the end of the .data
                                      // section then we are done.
          lw   a3, 0(a2)              // a3 = *a2. Load value from initial values into a3.
          sw   a3, 0(a0)              // *a0 = a3. Store initial value into
                                      // next place in .data.
          addi a0, a0, 4              // a0 = a0 + 4. Increment to next word in memory.
          addi a2, a2, 4              // a2 = a2 + 4. Increment to next word in flash.
          j data_init_loop            // Continue the loop.
      
        data_init_done:
    
        // With that initial setup out of the way, we now branch to the main
        // code in main.rs.
        j main
            ",
        gp = sym __global_pointer,
        estack = sym _estack,
        sbss = sym _sbss,
        ebss = sym _ebss,
        sdata = sym _sdata,
        edata = sym _edata,
        etext = sym _sidata,
        options(noreturn)
        );
    }
}

/// Registers saved in trap handler
#[allow(missing_docs)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
}

// TODO: LEFT OFF HERE:
// https://github.com/tock/tock/blob/master/arch/rv32i/src/lib.rs#L193
// and line 18 of asm.S

/// Trap entry point
///
/// `mcause` is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is examined and ExceptionHandler
/// or one of the core interrupt handlers is called.
#[link_section = ".trap"]
#[export_name = "_start_trap"]
#[naked]
pub extern "C" fn _start_trap() {
    unsafe {
        asm!("nop", options(noreturn));
    }
    // pub extern "C" fn _start_trap(trap_frame: *const TrapFrame) {
    // extern "C" {
    // fn ExceptionHandler(trap_frame: &TrapFrame);
    // fn DefaultHandler();
    // }

    // unsafe {
    // DefaultHandler();
    // }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultExceptionHandler(trap_frame: &TrapFrame) -> ! {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultInterruptHandler() {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

/* Interrupts */
#[doc(hidden)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    MachineSoft,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
}

pub use self::Interrupt as interrupt;

extern "C" {
    fn UserSoft();
    fn SupervisorSoft();
    fn MachineSoft();
    fn UserTimer();
    fn SupervisorTimer();
    fn MachineTimer();
    fn UserExternal();
    fn SupervisorExternal();
    fn MachineExternal();
}

#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[doc(hidden)]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 12] = [
    Vector { handler: UserSoft },
    Vector {
        handler: SupervisorSoft,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineSoft,
    },
    Vector { handler: UserTimer },
    Vector {
        handler: SupervisorTimer,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineTimer,
    },
    Vector {
        handler: UserExternal,
    },
    Vector {
        handler: SupervisorExternal,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineExternal,
    },
];

#[export_name = "abort"]
pub extern "C" fn abort() -> ! {
    loop {}
}
