use core::panic::PanicInfo;
use crate::{cpu, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  if let Some(args) = info.message() {
    println!("\nKernel panic: {}", args);
  } else {
    println!("\nKernel panic!");
  }

  cpu::wait_forever()
}
