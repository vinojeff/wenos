.section .text.boot

.global _start
_start:
    ldr x0, =_stack_top
    mov sp, x0
    bl rust_main
loop_hlt:
    wfi
    b loop_hlt