#[cfg(target_arch = "x86_64")]
#[path = "../arch/x86_64/cpu/boot.rs"]
pub mod boot;
#[cfg(target_arch = "aarch64")]
#[path = "../arch/aarch64/cpu/boot.rs"]
pub mod boot;