#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod cpu;
mod bsp;
mod panic_wait;
mod io;

unsafe fn kernel_main() -> ! {
  println!("[0] Hello from Rust!");

  panic!("Stopping here.")
}
