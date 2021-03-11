SECTIONS
{
    . = 0x80000;

    .text :
    {
        *(.text._start) *(.text*)
    }

    .rodata :
    {
        *(.rodata*)
    }

    .bss ALIGN(8):
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(8);

        . += 8
        __bss_end_inclusive = . - 8;
    }

    /DISCARD/ : { *(.comment*) }
}