
use core::{time::Duration};

use alloc::format;
use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_9X18_BOLD}, pixelcolor::Rgb888, prelude::{Point, RgbColor}, text::Text, Drawable, draw_target::DrawTarget, primitives::Rectangle};

use crate::bsp::framebuffer::FrameBuffer;

use super::UiInterface;

#[derive(Default)]
pub struct StartInterface {
  pub fps: f32,
}

impl UiInterface for StartInterface {
  fn draw(&mut self, fb: &mut FrameBuffer) {
    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb888::WHITE);
    let fps = format!("FPS: {}", self.fps);
    fb.fill_solid(&Rectangle::with_corners(Point::new(0, 0), Point::new(fb.width as i32 - 1, 36)), Rgb888::BLACK).unwrap();
    Text::new(&fps, Point::new(15, 18), style).draw(fb).unwrap();
  }

  fn on_input(&mut self) {}
  fn on_tick(&mut self, dt: Duration) {
    self.fps = 1.0 / dt.as_secs_f32()
  }

  fn should_draw(&self) -> bool {
    true
  }
}
