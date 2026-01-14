//! A panic handler that infinitely waits.

use core::panic::PanicInfo;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(target_arch = "x86_64")]
    loop {
        unsafe { core::arch::asm!("hlt") };
    }

    #[cfg(target_arch = "aarch64")]
    loop {
        unsafe { core::arch::asm!("wfi") };
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    loop {
        unsafe { core::hint::spin_loop() };
    }
}
