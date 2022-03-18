use core::time::Duration;

use crate::bsp::framebuffer::FrameBuffer;

pub trait UiInterface {
  fn draw(&mut self, fb: &mut FrameBuffer);
  fn should_draw(&self) -> bool;
  fn on_input(&mut self);
  fn on_tick(&mut self, dt: Duration);
}

pub fn get_ui_entrypoint() -> impl UiInterface {
  StartInterface::default()
}

// Reexports

mod start;
pub use start::*;
