MEMORY
{
    FLASH : ORIGIN = 0x0000, LENGTH = 32K
    RAM   : ORIGIN = 0x0100, LENGTH = 2K
}

/* Define sections and where they go */
SECTIONS
{
    .text :
    {
        *(.text*)
        *(.rodata*)
        *(.rodata*)
    } > FLASH

    .data : AT(ADDR(.text) + SIZEOF(.text))
    {
        *(.data*)
    } > RAM

    .bss :
    {
        *(.bss*)
        *(COMMON)
    } > RAM

    /* Provide a stack top */
    .stack ORIGIN(RAM) + LENGTH(RAM) :
    {
        _stack_start = .;
    }
}
