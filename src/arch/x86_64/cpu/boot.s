.section .note
.align 4
	.long 3      # name size ("Xen")
	.long 8      # data size (64-bit entry address)
	.long 18     # type (0x12 = Xen entry note)
.align 4
	.byte 'X','e','n'  # name
	.byte 0            # null terminator (不算在 name size 里)
.align 4
	.quad _start       # data - 64-bit entry address

.section .text._start
    .global _start

_start:

loop_hlt:
    hlt
    jmp loop_hlt