#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

use core::time::Duration;

use crate::graphics::init_fb;
use crate::graphics::ui::{get_ui_entrypoint, UiInterface};
use crate::mem::init_heap;
use crate::time::interface::TimeManager;
use crate::time::time_manager;

extern crate alloc;

mod bsp;
mod cpu;
mod graphics;
mod io;
mod mem;
mod panic_wait;
mod time;

const TARGET_FPS: u32 = 60;
const TARGET_DT: f32 = 1.0 / TARGET_FPS as f32;

unsafe fn kernel_main() -> ! {
  // Init Heap
  init_heap();

  info!("Hello from Rust!");

  let mut fb = init_fb();
  let mut current_ui = get_ui_entrypoint();

  let mut last_time = time_manager().uptime();

  loop {
    let mut dt = time_manager().uptime() - last_time;
    let diff = TARGET_DT - dt.as_secs_f32();
    if diff > 0.0 {
      time_manager().spin_for(Duration::from_secs_f32(diff));
      dt = time_manager().uptime() - last_time;
    }

    last_time = time_manager().uptime();
    current_ui.on_tick(dt);
    if current_ui.should_draw() {
      current_ui.draw(&mut fb);
    }

    fb.update_fb();
  }
}
