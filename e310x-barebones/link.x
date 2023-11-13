/* 
    OUTPUT_FORMAT configures the linker to use a platform-specific BFD
    backend to generate ELF files. BFD backends instruct the linker on 
    how to properly create the ELF sections for a given platform. 
    The list of acceptable values can be obtained using `objdump -i`.
*/
OUTPUT_FORMAT("elf32-littleriscv", "elf32-littleriscv", "elf32-littleriscv")

/* Variablies used in `asm.S` */
PROVIDE(_stext = ORIGIN(REGION_TEXT));
PROVIDE(_max_hart_id = 0);

/* TODO: IMPLEMENT THIS FUNCITON THEN REMOVE THIS */
PROVIDE(_setup_interrupts = default_setup_interrupts);

SECTIONS
{

  .stack (NOLOAD) :
  {
      /* Kernel stack.
       *
       * Place the kernel stack at the bottom of SRAM so that a
       * memory fault wil trigger if we exceed allocated stack space,
       * rather than silently overwriting valuable data.
       */
      . = ALIGN(8);
       _sstack = .;

       /* For GNU LD, we can just advance the location pointer (".") here to
        * reserve space for the stack. That, however, doesn't seem to work
        * for LLVM LLD. The resulting ELF has a stack section that shows the
        * correct size, but the next section (in our case .relocate) is not
        * moved down as well, instead it sits at the same address as .stack.
        * To work around this, we declare a dummy buffer and then insert it
        * here in the .stack section. This sets the stack size correctly and
        * places the .relocate section at the correct address. */
       /* See `STACK_BUFFER` in `main.rs` */
       KEEP(*(.stack_buffer))
       /*. = . + 0x800;*/  /*This is the original method. */

       . = ALIGN(8);
       _estack = .;
  } > REGION_STACK

  .text.dummy (NOLOAD) :
  {
    /* This section is intended to make _stext address (in memeory.x) work */
    . = ABSOLUTE(_stext);
  } > REGION_TEXT

  /* Program code and read-only data */
  .text _stext :
  {
    /* Segment must be 4-byte aligned */
    . = ALIGN(4);
    /* Put reset handler first in .text section so it ends up as the entry
       point of the program. See `.section .init` in `asm.S` */
    KEEP(*(.start));
    /* For RISC-V we need the `_start_trap` function to be 256 byte aligned,
     * and that function is at the start of the .riscv.trap section. If that
     * function does not exist (as for non-RISC-V platforms) then we do not
     * need any unusual alignment.
     * The allignment is implementation specific, so we currently use 256 to
     * work with the lowRISC CPUs.
     */
    . = DEFINED(_start_trap) ? ALIGN(256) : ALIGN(1);
    (*(.trap));

    *(.text .text.*);
  } > REGION_TEXT

  .rodata : ALIGN(4)
  {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
  } > REGION_RODATA

  .data : ALIGN(4)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    /* Must be called __global_pointer$ for linker relaxations to work. */
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.sdata .sdata.* .sdata2 .sdata2.*);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > REGION_DATA AT > REGION_RODATA

  .bss (NOLOAD) :
  {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > REGION_BSS

  /* Discard RISC-V relevant .eh_frame, we are not doing unwind on panic
     so it is not needed. */
  /DISCARD/ :
  {
    *(.eh_frame);
  }
}

ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
ERROR(linker): the start of the REGION_TEXT must be 4-byte aligned");

ASSERT(ORIGIN(REGION_RODATA) % 4 == 0, "
ERROR(linker): the start of the REGION_RODATA must be 4-byte aligned");

ASSERT(ORIGIN(REGION_DATA) % 4 == 0, "
ERROR(linker): the start of the REGION_DATA must be 4-byte aligned");

ASSERT(_stext % 4 == 0, "
ERROR(linker): `_stext` must be 4-byte aligned");

ASSERT(_sdata % 4 == 0 && _edata % 4 == 0, "
BUG(linker): .data is not 4-byte aligned");

ASSERT(_sidata % 4 == 0, "
BUG(linker): the LMA of .data is not 4-byte aligned");

ASSERT(_sbss % 4 == 0 && _ebss % 4 == 0, "
BUG(linker): .bss is not 4-byte aligned");

ASSERT(_stext + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "
ERROR(linker): The .text section must be placed inside the REGION_TEXT region.
Set _stext to an address smaller than 'ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT)'");
