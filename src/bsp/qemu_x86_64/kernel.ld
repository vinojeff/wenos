
OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)

__load_addr = 0x7C00;


ENTRY(_start)

/* Flags:
 *     4 == R
 *     5 == RX
 *     6 == RW
 */
PHDRS
{
    segment_code PT_LOAD FLAGS(5);
}

SECTIONS
{
    .note :
	{
		*(.note)
	}
    . =  __load_addr;

    /***********************************************************************************************
    * Code
    ***********************************************************************************************/
    .text :
    {
        KEEP(*(.text._start))
    } :segment_code
    
}
