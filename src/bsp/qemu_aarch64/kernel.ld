
OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH(aarch64)
__load_addr = 0x40080000;

/* Flags:
 *     4 == R
 *     5 == RX
 *     6 == RW
 */
PHDRS
{
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
}
SECTIONS
{
    . =  __load_addr;

    /***********************************************************************************************
    * Code
    ***********************************************************************************************/
    .text :
    {
        KEEP(*(.text.boot))
        KEEP(*(.text))
        KEEP(*(.text.*))
        *(.text.startup)
        /* stack use*/
        . = ALIGN(16);
            PROVIDE(_stack_start = .);
        . += 4K;
        . = ALIGN(16);
        PROVIDE(_stack_top = .);
    } :text
    .rodata : { *(.rodata .rodata.*) } :rodata
    .data : { *(.data .data.*) } :data
    .bss : { *(.bss .bss.*) }
}