#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_waiter;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    #[cfg(target_arch = "aarch64")]
    {
        use crate::bsp::qemu_aarch64::serial::Pl011Uart;
        use crate::bsp::qemu_aarch64::serial::simple_uart_puts;

        // 先用简单串口输出确认系统工作
        simple_uart_puts("\r\n");
        simple_uart_puts("Hello from WeNoS on QEMU aarch64!\r\n");
        simple_uart_puts("UART serial driver initialized.\r\n");
        simple_uart_puts("\r\n");

        // 测试 PL011Uart
        let mut uart = unsafe { Pl011Uart::new(0x0900_0000) };
        uart.init(24_000_000, 115200);

        uart.write_str("Kernel is running...\r\n");
    }

    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}