# Resources:
# https://sourceware.org/binutils/docs/as/CFI-directives.html
# https://mullerlee.cyou/2020/07/09/riscv-exception-interrupt/

# Entry point of all programs (_start).
# It initializes DWARF call frame information, the stack pointer, the
# frame pointer (needed for closures to work in start_rust) and the global
# pointer. Then it calls _start_rust.

# Trap entry point (_start_trap)
# Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
# restores caller saved registers and then returns.
.section .trap, "ax"
.global _start_trap
# Make it .weak so we can optionally provide our own if needed.
.weak _start_trap

_start_trap:
    addi sp, sp, -16*4

    sw ra, 0*4(sp)
    sw t0, 1*4(sp)
    sw t1, 2*4(sp)
    sw t2, 3*4(sp)
    sw t3, 4*4(sp)
    sw t4, 5*4(sp)
    sw t5, 6*4(sp)
    sw t6, 7*4(sp)
    sw a0, 8*4(sp)
    sw a1, 9*4(sp)
    sw a2, 10*4(sp)
    sw a3, 11*4(sp)
    sw a4, 12*4(sp)
    sw a5, 13*4(sp)
    sw a6, 14*4(sp)
    sw a7, 15*4(sp)

    add a0, sp, zero
    jal ra, _start_trap_rust

    lw ra, 0*4(sp)
    lw t0, 1*4(sp)
    lw t1, 2*4(sp)
    lw t2, 3*4(sp)
    lw t3, 4*4(sp)
    lw t4, 5*4(sp)
    lw t5, 6*4(sp)
    lw t6, 7*4(sp)
    lw a0, 8*4(sp)
    lw a1, 9*4(sp)
    lw a2, 10*4(sp)
    lw a3, 11*4(sp)
    lw a4, 12*4(sp)
    lw a5, 13*4(sp)
    lw a6, 14*4(sp)
    lw a7, 15*4(sp)

    addi sp, sp, 16*4
    mret

.section .text
.global default_setup_interrupts

default_setup_interrupts:
    # Set trap handler
    la t0, _start_trap
    csrw mtvec, t0
    ret

# Make sure there is an abort when linking
.globl abort
abort:
    j abort
