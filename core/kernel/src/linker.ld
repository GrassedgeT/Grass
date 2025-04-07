OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;

    .text : {
        .stext = .;
        *(.text.entry)
        . = ALIGN(4K);
        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(4K);
        *(.text .text.*)
    }

    .rodata ALIGN(4K) : {
        etext = .;
        srodata = .;
        *(.rodata .rodata.*)
    }

    .data ALIGN(4K) : {
        erodata = .;
        *(.data.stack)
        sdata = .;
        *(.data .data.*)
    }

    .bss ALIGN(4K) : {
        estack = .;
        sbss = .;
        *(.sbss .bss .bss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;
    PROVIDE(end = .);
}