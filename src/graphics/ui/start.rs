use alloc::format;
use core::time::Duration;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::ascii::FONT_9X18_BOLD;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Dimensions, Point, RgbColor};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

use super::UiInterface;
use crate::bsp::framebuffer::FrameBuffer;

#[derive(Default)]
pub struct StartInterface {
  pub fps: f32,
}

impl UiInterface for StartInterface {
  fn draw(&mut self, fb: &mut FrameBuffer) {
    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb888::WHITE);
    let (x, mut y) = (15, 15 + 9);

    let title_text = Text::new("Rusty Game OS", Point::new(x, y), style);
    fb.fill_solid(&title_text.bounding_box(), Rgb888::BLACK).unwrap();
    y += 20;
    title_text.draw(fb).unwrap();

    let fps = format!("FPS: {:.2}", self.fps);
    let text = Text::new(&fps, Point::new(x, y), style);
    fb.fill_solid(&text.bounding_box(), Rgb888::BLACK).unwrap();
    text.draw(fb).unwrap();
  }

  fn on_input(&mut self) {}

  fn on_tick(&mut self, dt: Duration) {
    self.fps = 1.0 / dt.as_secs_f32()
  }

  fn should_draw(&self) -> bool {
    true
  }
}
