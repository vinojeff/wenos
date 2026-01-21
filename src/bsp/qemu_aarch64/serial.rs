// 简化的串口驱动 - QEMU aarch64 virt UART
use core::ptr::{read_volatile, write_volatile};

// QEMU aarch64 virt UART 基地址
pub const UART_BASE: usize = 0x0900_0000;

// 寄存器偏移（与 PL011 手册一致）
const UART_DR:   usize = 0x00;
const UART_FR:   usize = 0x18;
const UART_IBRD: usize = 0x24;
const UART_FBRD: usize = 0x28;
const UART_LCRH: usize = 0x2C;
const UART_CR:   usize = 0x30;

// 标志位
const FR_TXFF: u32 = 1 << 5; // 发送缓冲区满

pub struct Pl011Uart {
    base: usize,
}

impl Pl011Uart {
    pub unsafe fn new(base: usize) -> Self {
        Self { base }
    }

    pub fn init(&mut self, clock_hz: u32, baud: u32) {
        // 1. 暂时禁用 UART
        self.write_reg(UART_CR, 0);

        // 2. 设置波特率（整数 + 小数分频）
        // div = clock / (16 * baud)
        // 对于 QEMU virt，24MHz / (16 * 115200) ≈ 13.02
        // div_int = 13, div_frac = (0.02 * 64) ≈ 1
        let div = (clock_hz as u64 * 64) / ((baud as u64) * 16);
        let div_int = (div / 64) as u32;
        let div_frac = (div % 64) as u32;

        self.write_reg(UART_IBRD, div_int);
        self.write_reg(UART_FBRD, div_frac);

        // 3. 设置数据格式：8N1，使能 FIFO
        let lcrh = (1 << 4) | (1 << 5) | (1 << 6); // WLEN=0b11, FEN=1
        self.write_reg(UART_LCRH, lcrh);

        // 4. 使能 UART 和发送/接收
        let cr = (1 << 0) | (1 << 8) | (1 << 9); // UARTEN, TXE, RXE
        self.write_reg(UART_CR, cr);
    }

    pub fn putc(&mut self, c: u8) {
        // 直接写入 PL011 UART 数据寄存器
        // QEMU virt 的 UART 工作得很快，不需要等待
        self.write_reg(UART_DR, c as u32);
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.bytes() {
            self.putc(c);
        }
    }

    #[inline]
    pub fn write_reg(&mut self, reg: usize, value: u32) {
        unsafe {
            let addr = self.base + reg;
            let v = value as u64;
            core::arch::asm!(
                "str {0}, [{1}]",
                in(reg) v,
                in(reg) addr,
            );
        }
    }
}

// QEMU aarch64 virt UART 基地址
const UART0_DR: u64 = 0x09000000;

/// 简单的串口输出（不初始化）
pub fn simple_uart_putc(c: u8) {
    unsafe {
        let v = c as u64;
        core::arch::asm!(
            "strb {0:w}, [{1}]",
            in(reg) v,
            in(reg) UART0_DR,
        );
    }
}

pub fn simple_uart_puts(s: &str) {
    for byte in s.bytes() {
        simple_uart_putc(byte);
    }
}
