use cortex_a::asm;
use core::arch::asm;
pub use bare_metal::{CriticalSection, Mutex};

/// Stop execution on core.
#[inline(always)]
pub fn wait_forever() -> ! {
  loop {
    asm::wfe()
  }
}

#[inline(always)]
pub fn disable_interrupts() {
  #[cfg(target_arch = "aarch64")]
  unsafe {
    asm!("msr daifset, #15", options(nomem, nostack, preserves_flags))
  }

  #[cfg(not(target_arch = "aarch64"))]
  unimplemented!()
}

#[inline(always)]
pub fn enable_interrupts() {
  #[cfg(target_arch = "aarch64")]
  unsafe {
    asm!("msr daifset, #0", options(nomem, nostack, preserves_flags))
  }

  #[cfg(not(target_arch = "aarch64"))]
  unimplemented!()
}

#[inline]
pub fn free<F, R>(f: F) -> R where F: FnOnce(&CriticalSection) -> R {
  //disable_interrupts();
  let r = f(unsafe { &CriticalSection::new() });
  //enable_interrupts();
  r
}
