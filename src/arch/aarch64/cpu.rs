use cortex_a::asm;

/// Stop execution on core.
#[inline(always)]
pub fn wait_forever() -> ! {
  loop {
    asm::wfe()
  }
}
