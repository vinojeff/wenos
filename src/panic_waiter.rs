//! A panic handler that infinitely waits.

use core::panic::PanicInfo;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(target_arch = "x86_64")]
    {
        use crate::bsp::qemu_x86_64::serial::puts;
        puts("\nPANIC: ");
        puts(info.to_string().as_str());
        puts("\n");
        loop {
            unsafe { core::arch::asm!("hlt") };
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        use crate::bsp::qemu_aarch64::serial::Pl011Uart;
        use crate::bsp::qemu_aarch64::serial::UART_BASE;
        let mut uart = unsafe { Pl011Uart::new(UART_BASE) };
        uart.init(24_000_000, 115200);
        uart.write_str("\r\n");
        uart.write_str("\nPANIC: ");
        // 简单的 panic 信息输出
        if let Some(location) = info.location() {
            let msg = format_args!("{}:{}: {}",
                location.file(),
                location.line(),
                info.message()
            );
            // 这里暂时简化处理
            uart.write_str("panic occurred\n");
        } else {
            uart.write_str("panic occurred (no location)\n");
        }
        uart.write_str("\n");
        loop {
            unsafe { core::arch::asm!("wfi") };
        }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    loop {
        unsafe { core::hint::spin_loop() };
    }
}
