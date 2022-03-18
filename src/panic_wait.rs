use core::panic::PanicInfo;
use crate::{cpu, panic_println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  if let Some(args) = info.message() {
    panic_println!("\nKernel panic: {}", args);
  } else {
    panic_println!("\nKernel panic!");
  }

  cpu::wait_forever()
}
