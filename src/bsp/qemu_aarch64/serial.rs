// 简化的串口驱动 - QEMU aarch64 virt UART
use core::fmt;

// QEMU aarch64 virt UART 基地址
const UART0_DR: u64 = 0x09000000;

/// 串口输出单个字符
pub fn putc(c: u8) {
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
pub fn puts(s: &str) {
    for byte in s.bytes() {
        putc(byte);
    }
}

/// UART 驱动结构体
pub struct Uart;

impl Uart {
    pub const fn new() -> Self { Uart }
}

/// 实现 fmt::Write trait 以支持格式化输出
impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

// 全局 UART 实例
pub static UART: Uart = Uart::new();

/// 格式化输出宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut uart = $crate::bsp::qemu_aarch64::serial::UART;
        write!(uart, $($arg)*).unwrap();
    });
}

/// 格式化输出并换行宏
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut uart = $crate::bsp::qemu_aarch64::serial::UART;
        writeln!(uart, $($arg)*).unwrap();
    });
}
