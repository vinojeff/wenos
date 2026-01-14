#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_waiter;

//use core::arch::global_asm;

// // 直接包含汇编文件
// global_asm!(include_str!("arch/aarch64/cpu/boot.s"));

// #[unsafe(no_mangle)]
// pub extern "C" fn rust_main() -> ! {
//     loop {
//         unsafe { core::arch::asm!("wfi") };
//     }
// }