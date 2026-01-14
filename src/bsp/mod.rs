#[cfg(any(target_arch = "x86_64"))]
pub mod qemu_x86_64;
#[cfg(any(target_arch = "aarch64"))]
pub mod qemu_aarch64;
#[cfg(any(target_arch = "riscv64"))]
pub mod qemu_riscv64;