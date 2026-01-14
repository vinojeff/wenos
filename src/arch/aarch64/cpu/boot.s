.section .text.boot

.global _start
_start:
loop_hlt:
    wfi
    b loop_hlt