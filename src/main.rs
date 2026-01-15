#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_waiter;

// QEMU aarch64 virt UART 基地址
const UART0_DR: u64 = 0x09000000;

/// 串口输出单个字符
pub fn uart_putc(c: u8) {
    unsafe {
        let v = c as u64;
        core::arch::asm!(
            "strb {0:w}, [{1}]",
            in(reg) v,
            in(reg) UART0_DR,
        );
    }
}

/// 串口输出字符串
pub fn uart_puts(s: &str) {
    for byte in s.bytes() {
        uart_putc(byte);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    #[cfg(target_arch = "aarch64")]
    {
        uart_puts("\r\n");
        uart_puts("Hello from WeNoS on QEMU aarch64!\r\n");
        uart_puts("UART serial driver initialized.\r\n");

        // 测试单个字符输出
        uart_putc(b'A');
        uart_putc(b'B');
        uart_putc(b'C');
        uart_puts("\r\n");

        uart_puts("Kernel is running...\r\n");
    }

    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}